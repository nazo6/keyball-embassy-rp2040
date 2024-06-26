#![allow(clippy::single_match)]

use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use crate::{
    config::{COLS, LAYER_NUM, TAP_THRESHOLD},
    driver::keyboard::{Hand, KeyChangeEventOneHand},
    keyboard::keycode::{KeyCode, Layer},
    state::{common::CommonLocalState, manager::interface::LocalStateManager as _},
};

use self::{
    common::CommonState,
    pressed::{AllPressed, KeyStatusChangeType, KeyStatusUpdateEvent},
};

use super::keyboard::keycode::KeyAction;

mod common;
mod manager;
mod pressed;

pub struct StateReport {
    pub keyboard_report: Option<KeyboardReport>,
    pub mouse_report: Option<MouseReport>,
    pub media_keyboard_report: Option<MediaKeyboardReport>,
    pub highest_layer: u8,
}

pub struct State {
    common_state: CommonState,
    master_hand: Hand,
    pressed: AllPressed,
    mouse: manager::mouse::MouseState,
    keyboard: manager::keyboard::KeyboardState,
    media_keyboard: manager::media_keyboard::MediaKeyboardState,
}

macro_rules! process_event {
    ($cs:expr, $cls:expr, $kc:expr, $event:expr, ($s1:expr, $s1g:expr)) => {
        $s1.process_event($cs, $cls, $s1g, $kc, $event)
    };
    ($cs:expr, $cls:expr, $kc:expr, $event:expr, ($s1:expr, $s1g:expr), $(($s:expr, $sg:expr)),+) => {
        $s1.process_event($cs, $cls, $s1g, $kc, $event);
        process_event!($cs, $cls, $kc, $event, $(($s, $sg)),+);
    };
}

macro_rules! loop_end {
    ($cs:expr, $cls:expr, ($s1:expr, $s1g:expr)) => {
        $s1.loop_end($cs, $cls, $s1g)
    };
    ($cs:expr, $cls:expr, ($s1:expr, $s1g:expr), $(($s:expr, $sg:expr)),+) => {
        $s1.loop_end($cs, $cls, $s1g);
        loop_end!($cs, $cls, $(($s, $sg)),+);
    };
}

impl State {
    pub fn new(layers: [Layer; LAYER_NUM], master_hand: Hand) -> Self {
        Self {
            master_hand,
            common_state: CommonState::new(layers),

            pressed: AllPressed::new(),

            mouse: manager::mouse::MouseState::new(),
            keyboard: manager::keyboard::KeyboardState::new(),
            media_keyboard: manager::media_keyboard::MediaKeyboardState::new(),
        }
    }

    pub fn update(
        &mut self,
        master_events: &mut [KeyChangeEventOneHand],
        slave_events: &mut [KeyChangeEventOneHand],
        mouse_event: (i8, i8),
    ) -> StateReport {
        let prev_highest_layer = self.common_state.highest_layer();

        let mut cls = CommonLocalState::new(prev_highest_layer);

        let mut mls = manager::mouse::MouseLocalState::new(mouse_event);
        let mut kls = manager::keyboard::KeyboardLocalState::new();
        let mut mkls = manager::media_keyboard::MediaKeyboardLocalState::new();
        let mut lls = manager::layer::LayerLocalState::new();

        let events = {
            let (left_events, right_events) = if self.master_hand == Hand::Left {
                (master_events, slave_events)
            } else {
                (slave_events, master_events)
            };
            right_events.iter_mut().for_each(|event| {
                event.col = ((COLS - 1) as u8 - event.col) + COLS as u8;
            });
            let both_events = right_events.iter().chain(left_events.iter());

            self.pressed
                .compose_events_and_update_pressed(both_events, cls.now)
        };

        for event in events.iter() {
            let Some(kc) = self.resolve_key(event, prev_highest_layer) else {
                continue;
            };

            process_event!(
                &mut self.common_state,
                &mut cls,
                &kc,
                event,
                (mls, &mut self.mouse),
                (kls, &mut self.keyboard),
                (mkls, &mut self.media_keyboard),
                (lls, &mut ())
            );
        }

        loop_end!(
            &mut self.common_state,
            &mut cls,
            (mls, &mut self.mouse),
            (kls, &mut self.keyboard),
            (mkls, &mut self.media_keyboard),
            (lls, &mut ())
        );

        StateReport {
            keyboard_report: kls.report(&self.common_state, &cls, &mut self.keyboard),
            mouse_report: mls.report(&self.common_state, &cls, &mut self.mouse),
            media_keyboard_report: mkls.report(&self.common_state, &cls, &mut self.media_keyboard),
            highest_layer: prev_highest_layer as u8,
        }
    }

    fn resolve_key(&mut self, event: &KeyStatusUpdateEvent, layer: usize) -> Option<KeyCode> {
        let key_action = self.common_state.get_keycode(event.row, event.col, layer)?;

        match event.change_type {
            KeyStatusChangeType::Pressed => match key_action {
                KeyAction::Tap(kc) => Some(kc),
                _ => None,
            },
            KeyStatusChangeType::Pressing(duration) => match key_action {
                KeyAction::Tap(kc) => Some(kc),
                KeyAction::TapHold(_tkc, hkc) => {
                    if duration > TAP_THRESHOLD {
                        Some(hkc)
                    } else {
                        None
                    }
                }
            },
            KeyStatusChangeType::Released(duration) => match key_action {
                KeyAction::Tap(kc) => Some(kc),
                KeyAction::TapHold(tkc, hkc) => {
                    if duration > TAP_THRESHOLD {
                        Some(hkc)
                    } else {
                        Some(tkc)
                    }
                }
            },
        }
    }
}
