mod inputs;
use crate::pros::misc;
pub use inputs::{Button, ControllerType, StickAxis};

pub struct Controller {
    controller_type: ControllerType,
}

impl Controller {
    pub fn new(controller_type: ControllerType) -> Self {
        Self { controller_type }
    }
    pub fn was_pressed(&self, button: Button) -> bool {
        unsafe {
            misc::controller_get_digital_new_press(
                self.controller_type.to_pros_controller_type(),
                button.to_pros_button(),
            ) == 1
        }
    }
    pub fn is_down(&self, button: Button) -> bool {
        unsafe {
            misc::controller_get_digital(
                self.controller_type.to_pros_controller_type(),
                button.to_pros_button(),
            ) == 1
        }
    }

    pub fn get_stick(&self, stick_axis: StickAxis) -> i32 {
        unsafe {
            misc::controller_get_analog(
                self.controller_type.to_pros_controller_type(),
                stick_axis.to_pros_stick_axis(),
            )
        }
    }

    pub fn print(&self, line: u8, text: *const i8) {
        unsafe {
            misc::controller_print(
                self.controller_type.to_pros_controller_type(),
                line,
                0,
                text
            );
        }
    }
}
