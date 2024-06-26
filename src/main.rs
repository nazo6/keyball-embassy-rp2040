#![no_std]
#![no_main]

use core::{
    panic::PanicInfo,
    sync::atomic::{self, Ordering},
};

use crate::{config::DOUBLE_TAP_THRESHOLD, utils::print};
use defmt_rtt as _;
use device::peripherals::init_peripherals;
use display::DISPLAY;
use embassy_executor::Spawner;
use task::TaskPeripherals;

mod config;
mod device;
mod display;
mod driver;
mod keyboard;
mod state;
mod task;
mod usb;
mod utils;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = init_peripherals();

    driver::double_tap::check_double_tap(DOUBLE_TAP_THRESHOLD).await;

    DISPLAY.init(peripherals.display).await;
    utils::print!("Initializing...");

    task::start(TaskPeripherals {
        ball: peripherals.ball,
        keyboard: peripherals.keyboard,
        split: peripherals.split,
        led: peripherals.led,
        usb: peripherals.usb,
        temp: peripherals.temp,
    })
    .await;
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use core::fmt::Write;

    let loc = info.location().unwrap();
    let file = loc.file().as_bytes();
    let file = if file.len() > 20 {
        &file[file.len() - 20..]
    } else {
        file
    };
    let file = core::str::from_utf8(file).unwrap();

    let mut str = heapless::String::<512>::new();
    write!(str, "\n{}\n{}\n", file, loc.line()).unwrap();
    crate::DISPLAY.try_draw_text(&str);

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
