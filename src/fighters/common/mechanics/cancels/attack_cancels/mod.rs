use super::*;

pub static mut ATTACK_CANCEL : [bool; 8] = [false; 8];
pub static mut ENABLE_ATTACK_CANCEL : [bool; 8] = [false; 8];
pub static mut ATTACK_CANCEL_COUNTER : [bool; 8] = [false; 8];
pub static mut ATTACK_CANCEL_COUNT : [i32; 8] = [0; 8];
pub static mut ENABLE_MULTIHIT_CANCEL : [bool; 8] = [false; 8];
pub static mut MOVEMENT_CANCEL : [bool; 8] = [true; 8];
pub static mut ENABLE_MULTIHIT_COUNT : [bool; 8] = [true; 8];
pub static mut MULTIHIT : [bool; 8] = [false; 8];
pub static mut MULTIHIT_COUNT : [i32; 8] = [0; 8];
pub static mut IS_SUCCESSFUL_HIT : [bool; 8] = [false; 8];

pub mod cancel_on_hit;
pub mod common_cancels;
pub mod resets_falses;

pub fn install() {
    common_cancels::install();
    resets_falses::install();
}
