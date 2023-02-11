#![allow(dead_code)]

use crate::pros::motors;
pub struct Motor {
    velocity: i32,
    port: u8,
    brake_mode: BrakeMode,
    gear_set: GearSet,
    reversed: bool,
}

pub enum BrakeMode {
    Coast,
    Brake,
    Hold,
}
impl BrakeMode {
    fn to_pros_brakemode(&self) -> motors::motor_brake_mode_e_t {
        match self {
            Self::Coast => motors::motor_brake_mode_e_E_MOTOR_BRAKE_COAST,
            Self::Brake => motors::motor_brake_mode_e_E_MOTOR_BRAKE_BRAKE,
            Self::Hold => motors::motor_brake_mode_e_E_MOTOR_BRAKE_HOLD,
        }
    }
}

pub enum GearSet {
    Red,
    Green,
    Blue,
}
impl GearSet {
    fn to_pros_gearset(&self) -> motors::motor_gearset_e {
        match self {
            Self::Red => motors::motor_gearset_e_E_MOTOR_GEAR_RED,
            Self::Green => motors::motor_gearset_e_E_MOTOR_GEAR_GREEN,
            Self::Blue => motors::motor_gearset_e_E_MOTOR_GEAR_BLUE,
        }
    }
}

impl Motor {
    pub fn new(port: i8, brake_mode: BrakeMode, gear_set: GearSet) -> Self {
        let reversed = port.is_negative();
        let port: u8 = port.abs() as u8;
        let velocity = 0;
        unsafe {
            motors::motor_set_brake_mode(port, brake_mode.to_pros_brakemode());
            motors::motor_set_gearing(port, gear_set.to_pros_gearset());
            motors::motor_set_reversed(port, reversed);
        }
        Self {
            velocity,
            port,
            reversed,
            brake_mode,
            gear_set,
        }
    }

    pub fn spin(&mut self, velocity: i32) {
        self.velocity = velocity;
        unsafe {
            motors::motor_move(self.port, self.velocity);
        }
    }

    pub fn brake(&mut self) {
        self.velocity = 0;
        unsafe {
            motors::motor_brake(self.port);
        }
    }
}
