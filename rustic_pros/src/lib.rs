#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

use panic_halt as _;
mod controller;
mod libc;
mod motor;
mod pros;
use libc::stdio;
fn sleep(millis: u32) {
    unsafe {
        pros::rtos::delay(millis);
    }
}
fn print(text: &str) {
    unsafe {
        libc::stdio::printf(text.as_ptr().cast());
    }
}

#[no_mangle]
pub extern "C" fn rust_initalize() {}

#[no_mangle]
pub extern "C" fn rust_autonomous() {
    loop {
        let c = controller::Controller::new(controller::ControllerType::Primary);
        let mut m = motor::Motor::new(1, motor::BrakeMode::Hold, motor::GearSet::Blue);
        if c.is_down(controller::Button::A) {
            m.spin(-60);
            print("down\n");
        } else {
            m.brake();
            print("up\n");
        }
        sleep(200);
    }
}
#[no_mangle]
pub extern "C" fn rust_disabled() {}
#[no_mangle]
pub extern "C" fn rust_usercontrol() {
    loop {
        let c = controller::Controller::new(controller::ControllerType::Primary);
        let mut m = motor::Motor::new(1, motor::BrakeMode::Hold, motor::GearSet::Blue);
        if c.is_down(controller::Button::B) {
            m.spin(100);
            print("down\n");
        } else {
            m.brake();
            print("up\n");
        }
        sleep(200);
    }
}
