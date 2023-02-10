use crate::pros::misc;

pub enum ControllerType {
    Primary,
    Secondary,
}
impl ControllerType {
    pub(super) fn to_pros_controller_type(&self) -> misc::controller_id_e_t {
        match self {
            Self::Primary => misc::controller_id_e_t_E_CONTROLLER_MASTER,
            Self::Secondary => misc::controller_id_e_t_E_CONTROLLER_PARTNER,
        }
    }
}

pub enum Button {
    A,
    B,
    X,
    Y,
    Up,
    Down,
    Left,
    Right,
    L1,
    L2,
    R1,
    R2,
}
impl Button {
    pub(super) fn to_pros_button(&self) -> misc::controller_digital_e_t {
        match self {
            Self::A => misc::controller_digital_e_t_E_CONTROLLER_DIGITAL_A,
            Self::B => misc::controller_digital_e_t_E_CONTROLLER_DIGITAL_B,
            Self::X => misc::controller_digital_e_t_E_CONTROLLER_DIGITAL_X,
            Self::Y => misc::controller_digital_e_t_E_CONTROLLER_DIGITAL_Y,
            Self::Up => misc::controller_digital_e_t_E_CONTROLLER_DIGITAL_UP,
            Self::Down => misc::controller_digital_e_t_E_CONTROLLER_DIGITAL_DOWN,
            Self::Left => misc::controller_digital_e_t_E_CONTROLLER_DIGITAL_LEFT,
            Self::Right => misc::controller_digital_e_t_E_CONTROLLER_DIGITAL_RIGHT,
            Self::L1 => misc::controller_digital_e_t_E_CONTROLLER_DIGITAL_L1,
            Self::L2 => misc::controller_digital_e_t_E_CONTROLLER_DIGITAL_L2,
            Self::R1 => misc::controller_digital_e_t_E_CONTROLLER_DIGITAL_R1,
            Self::R2 => misc::controller_digital_e_t_E_CONTROLLER_DIGITAL_R2,
        }
    }
}
pub enum StickAxis {
    HorizontalLeft,
    VerticalLeft,
    HorizontalRight,
    VerticalRight,
}
impl StickAxis {
    pub(super) fn to_pros_stick_axis(&self) -> misc::controller_analog_e_t {
        match self {
            Self::HorizontalLeft => misc::controller_analog_e_t_E_CONTROLLER_ANALOG_LEFT_X,
            Self::VerticalLeft => misc::controller_analog_e_t_E_CONTROLLER_ANALOG_LEFT_Y,
            Self::HorizontalRight => misc::controller_analog_e_t_E_CONTROLLER_ANALOG_RIGHT_X,
            Self::VerticalRight => misc::controller_analog_e_t_E_CONTROLLER_ANALOG_RIGHT_Y,
        }
    }
}
