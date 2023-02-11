#![allow(dead_code)]

use crate::pros::motors;

pub struct Motor {
    velocity: i32,
    port: u8,
    brake_mode: BrakeMode,
    gear_set: GearSet,
    reversed: bool,
}
///how a motor should brake
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
///a motor gear cartrige
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
    ///create a new motor
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
    ///Gets the temperature of the motor in degrees Celsius. The resolution of this eading is 5 degrees Celsius. The motor will start to reduce its power when the temperature reading is greater than or equal to 55 C.
    pub fn temperature(&self) -> f64 {
        unsafe { motors::motor_get_temperature(self.port) }
    }
    ///Stops the motor using the currently configured brake mode.
    ///
    ///This function sets motor velocity to zero, which will cause it to act according to the set brake mode. If brake mode is set to MOTOR_BRAKE_HOLD, this function may behave differently than calling motor_move_absolute(0) or motor_move_relative(0).
    pub fn brake(&mut self) {
        self.velocity = 0;
        unsafe {
            motors::motor_brake(self.port);
        }
    }
    ///Sets the voltage for the motor
    pub fn spin_voltage(&self, voltage_percent: i32) {
        let voltage = voltage_percent * 12000 / 100;
        unsafe {
            motors::motor_move_voltage(self.port, voltage);
        }
    }
    ///sets the velocity of the motor in rpm
    pub fn spin_velocity(&self, velocity: i32) {
        unsafe {
            motors::motor_move_velocity(self.port, velocity);
        }
    }
    ///sets an amount of movement to be done by the motor as a background process
    pub fn spin_relative(&self, distance: i32) {
        unsafe {
            motors::motor_move_relative(self.port, self.position(), distance);
        }
    }
    ///query the motor's posiiton
    pub fn position(&self) -> f64 {
        unsafe { motors::motor_get_position(self.port) }
    }
}
