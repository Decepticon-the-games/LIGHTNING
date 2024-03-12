use super::*;

pub static mut FIGHTER_MANAGER_ADDR: usize = 0;
pub static mut ON_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING: [bool; 8] = [false; 8];
pub static mut IS_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING: [bool; 8] = [false; 8];
pub static mut OFF_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING: [bool; 8] = [false; 8];
pub static mut CAN_LIGHTNING: [bool; 8] = [true; 8];
pub static mut LIGHTNING_PRE : [bool; 8] = [false; 8];
pub static mut LIGHTNING_TIMER : [i32; 8] = [-1;  8];
pub static mut CAN_LIGHTNING_TEMP : [bool; 8] = [true; 8];
pub static mut LIGHTNING_BUTTON : [bool; 8] = [false; 8];
pub static mut LIGHTNING_EFFECTS: [bool; 8] = [false; 8];
pub static mut ONEFRAMEEFFECTS: [bool; 8] = [false; 8];

pub mod opff;
//ub mod status_kind;

pub fn install() {
    opff::install();
    //status_kind::install();
}