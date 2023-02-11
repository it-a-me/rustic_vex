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
    log::error!("errrrror");
    log::trace!("ofund");
    loop {
        let c = controller::Controller::new(controller::ControllerType::Primary);
        let mut m = motor::Motor::new(1, motor::BrakeMode::Coast, motor::GearSet::Blue);
        if c.is_down(controller::Button::B) {
            m.spin(100);
        } else {
            m.brake();
        }
        sleep(200);
    }
}
