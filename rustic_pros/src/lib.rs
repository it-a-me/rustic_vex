#![no_std]
#![feature(alloc_error_handler)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

use panic_halt as _;
mod libc;
mod pros;
use libc::stdio;

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;
use core::alloc::Layout;
use core::panic::PanicInfo;
use embedded_alloc::Heap;
use no_std_compat::prelude::v1::*;
#[global_allocator]
static HEAP: Heap = Heap::empty();

#[no_mangle]
pub extern "C" fn rust_initalize() {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
    let data = "data";
    let data_string = "data".to_string();
    unsafe {
        let pointy = data.as_ptr() as *const i8;
        let pointy_string = data_string.as_ptr() as *const i8;
        stdio::printf(pointy);
    }
}

#[no_mangle]
pub extern "C" fn rust_autonomous() {}
#[no_mangle]
pub extern "C" fn rust_usercontrol() {
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
