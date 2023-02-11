#![allow(dead_code)]

mod inputs;
use crate::pros::misc;
pub use inputs::{Button, ControllerType, StickAxis};
///wrapper around a pros controller
pub struct Controller {
    controller_type: ControllerType,
}

impl Controller {
    ///create a new controller
    pub fn new(controller_type: ControllerType) -> Self {
        Self { controller_type }
    }
    ///check if a button was pressed.  Returns true if the button has been pressed since it was
    ///last polled
    pub fn was_pressed(&self, button: Button) -> bool {
        unsafe {
            misc::controller_get_digital_new_press(
                self.controller_type.to_pros_controller_type(),
                button.to_pros_button(),
            ) == 1
        }
    }
    ///checks if a button is down
    pub fn is_down(&self, button: Button) -> bool {
        unsafe {
            misc::controller_get_digital(
                self.controller_type.to_pros_controller_type(),
                button.to_pros_button(),
            ) == 1
        }
    }
    ///the value of a stick at a given axis (between -100 and 100)
    pub fn get_stick(&self, stick_axis: StickAxis) -> i32 {
        let raw_stick_position = unsafe {
            misc::controller_get_analog(
                self.controller_type.to_pros_controller_type(),
                stick_axis.to_pros_stick_axis(),
            )
        };
        raw_stick_position * 100 / 127
    }
    pub fn print(&self, line: u8, text: &str) {
        if let Ok(printable) = cstr_core::CString::new(text) {
            unsafe {
                misc::controller_print(
                    self.controller_type.to_pros_controller_type(),
                    line,
                    0,
                    printable.as_ptr().cast(),
                );
            }
        } else {
            log::error!("failed to print message to screen\n'{text}'");
        }
    }
}
