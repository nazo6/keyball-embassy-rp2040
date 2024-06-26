use embassy_futures::yield_now;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::{Common, Config, Pin, Pio, ShiftDirection, StateMachine};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::semaphore::{FairSemaphore, Semaphore};
use embassy_time::Timer;

use fixed::traits::ToFixed;

use crate::config::SPLIT_CLK_DIVIDER;
use crate::device::interrupts::Irqs;
use crate::device::peripherals::SplitPeripherals;

// Data structure
//
// 0 bit: start
// 1-8 bit: data
// 9 bit: start check
// 10bit: end check

fn rx_init<'a>(
    common: &mut Common<'a, PIO0>,
    sm: &mut StateMachine<'a, PIO0, 0>,
    data_pin: &Pin<'a, PIO0>,
) {
    let prg = pio_proc::pio_asm!(
        "set pindirs 0",
        ".wrap_target",
        "wait 0 pin 0",
        "set x 9 [8]",
        "bitloop:",
        "in pins 1 [6]",
        "jmp x-- bitloop",
        "push",
        ".wrap"
    );
    let mut cfg = Config::default();
    cfg.use_program(&common.load_program(&prg.program), &[]);

    cfg.set_in_pins(&[data_pin]);

    cfg.shift_in.direction = ShiftDirection::Left;

    cfg.fifo_join = embassy_rp::pio::FifoJoin::RxOnly;

    cfg.clock_divider = SPLIT_CLK_DIVIDER.to_fixed();
    sm.set_config(&cfg);
    sm.set_enable(true);
}

fn tx_init<'a>(
    common: &mut Common<'a, PIO0>,
    sm: &mut StateMachine<'a, PIO0, 1>,
    data_pin: &Pin<'a, PIO0>,
) {
    let prg = pio_proc::pio_asm!(
        "set pindirs 0",
        ".wrap_target",
        "pull",
        "set x 9 [2]",
        "set pins 0",
        "set pindirs 1 [7]",
        "bitloop:",
        "out pins 1 [6]",
        "jmp x-- bitloop",
        "set pins 1",
        "set pindirs 0 [6]",
        ".wrap"
    );
    let mut cfg = Config::default();
    cfg.use_program(&common.load_program(&prg.program), &[]);

    cfg.set_out_pins(&[data_pin]);
    cfg.set_set_pins(&[data_pin]);

    cfg.shift_out.direction = ShiftDirection::Left;

    cfg.fifo_join = embassy_rp::pio::FifoJoin::TxOnly;

    cfg.clock_divider = SPLIT_CLK_DIVIDER.to_fixed();
    sm.set_config(&cfg);
    sm.set_enable(false);
}

pub struct Communicate<'a> {
    rx_sm: StateMachine<'a, PIO0, 0>,
    tx_sm: StateMachine<'a, PIO0, 1>,
    pin: Pin<'a, PIO0>,
}

static COMM_SEMAPHORE: FairSemaphore<ThreadModeRawMutex, 3> = FairSemaphore::new(1);

impl<'a> Communicate<'a> {
    pub async fn new<'b: 'a>(p: SplitPeripherals) -> Communicate<'a> {
        let pio = Pio::new(p.pio, Irqs);

        let mut common = pio.common;
        let mut sm0 = pio.sm0;
        let mut sm1 = pio.sm1;

        let mut out_pin = common.make_pio_pin(p.data_pin);
        out_pin.set_pull(embassy_rp::gpio::Pull::Up);

        rx_init(&mut common, &mut sm0, &out_pin);
        tx_init(&mut common, &mut sm1, &out_pin);

        let mut s = Self {
            rx_sm: sm0,
            tx_sm: sm1,
            pin: out_pin,
        };

        s.enter_rx().await;

        s
    }

    async fn enter_rx(&mut self) {
        while !self.tx_sm.tx().empty() {
            yield_now().await;
        }

        Timer::after_micros(300).await;

        self.tx_sm.set_enable(false);
        self.pin.set_drive_strength(embassy_rp::gpio::Drive::_2mA);
        self.rx_sm.restart();
        self.rx_sm.set_enable(true);
    }

    async fn enter_tx(&mut self) {
        self.rx_sm.set_enable(false);
        self.pin.set_drive_strength(embassy_rp::gpio::Drive::_12mA);
        self.tx_sm.restart();
        self.tx_sm.set_enable(true);
    }

    pub async fn recv_byte(&mut self) -> (bool, bool, u8) {
        let mut data = self.rx_sm.rx().wait_pull().await;
        let end_bit = data & 1;
        data >>= 1;
        let start_bit = data & 1;
        data >>= 1;
        (start_bit == 1, end_bit == 1, data as u8)
    }
    pub async fn recv_data<const N: usize>(&mut self, buf: &mut [u8; N]) {
        let _permit = loop {
            let (start, end, data) = self.recv_byte().await;

            let permit = COMM_SEMAPHORE.try_acquire(1);

            if !start {
                continue;
            }

            buf[0] = data;

            if end {
                return;
            }

            break permit;
        };

        for b in buf.iter_mut().skip(1) {
            let (_start, end, data) = self.recv_byte().await;

            *b = data;

            if end {
                break;
            }
        }
    }

    pub async fn send_data<const N: usize>(&mut self, buf: &[u8]) {
        let _permit = COMM_SEMAPHORE.acquire(1).await;

        self.enter_tx().await;

        for (i, data) in buf.iter().enumerate() {
            let mut data = (*data as u32) << 24;

            if i == 0 {
                data |= 1 << 23;
            }
            if i == buf.len() - 1 {
                data |= 1 << 22;
            }

            self.tx_sm.tx().wait_push(data).await;
        }

        self.enter_rx().await;
    }
}
