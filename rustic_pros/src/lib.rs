#![no_std]
use no_std_compat::prelude::v1::*;

use panic_halt as _;

mod controller;
mod libc;
mod motor;
mod pros;

mod vex_logger;
use vex_logger::VexLogger;
static LOGGER: VexLogger = VexLogger;

#[global_allocator]
static ALLOCATOR: emballoc::Allocator<4096> = emballoc::Allocator::new();

fn sleep(millis: u32) {
    unsafe {
        pros::rtos::delay(millis);
    }
}
#[no_mangle]
pub extern "C" fn rust_initalize() {
    vex_logger::init(log::LevelFilter::Trace);
}
#[no_mangle]
pub extern "C" fn rust_competition_initalize() {}

#[no_mangle]
pub extern "C" fn rust_autonomous() {}
#[no_mangle]
pub extern "C" fn rust_disabled() {}
#[no_mangle]
pub extern "C" fn rust_usercontrol() {
    let mut voltage = 0;
    loop {
        let c = controller::Controller::new(controller::ControllerType::Primary);
        let mut m = motor::Motor::new(1, motor::BrakeMode::Coast, motor::GearSet::Blue);
        if c.was_pressed(controller::Button::A) {
            let stick_pos = c.get_stick(controller::StickAxis::VerticalLeft);
            log::info!("voltage set to {}", stick_pos);
            voltage = stick_pos;
        }
        if c.is_down(controller::Button::B) {
            m.spin_voltage(voltage);
        } else {
            m.brake();
        }
        sleep(100);
    }
}
