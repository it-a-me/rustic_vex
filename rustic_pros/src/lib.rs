#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use panic_halt as _;

mod pros;

#[no_mangle]
pub extern "C" fn dong() {
    unsafe {
        if pros::misc::controller_get_digital(
            pros::misc::controller_id_e_t_E_CONTROLLER_MASTER,
            pros::misc::controller_digital_e_t_E_CONTROLLER_DIGITAL_L1,
        ) == 1
        {
            pros::motors::motor_move(1, 50);
        } else {
            pros::motors::motor_move(1, 0);
        }
    }
}
