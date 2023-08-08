use super::*;

pub static mut VANISH : [bool; 8] = [false; 8];
pub static mut VANISH_READY : [bool; 8] = [false; 8];
pub static mut VANISH_BUTTON : [bool; 8] = [false; 8]; // Only used to replicate a button_pad_trigger, runs only 1 frame
pub static mut CAN_VANISH : [bool; 8] = [false; 8];//Incorporating the ability to use vanish under condition. See lightning_01_motioncancels/mod.rs
pub static mut VANISH_COUNTER : [bool; 8] = [false; 8];
pub static mut VANISH_COUNT : [i32; 8] = [0; 8];//See motioncancels/mod.rs
pub static mut VANISH_RESET : [bool; 8] = [false; 8];//See motioncancels/mod.rs
pub static mut CANCEL_INTO_VANISH : [bool; 8] = [false; 8];
static mut CAMERA : [bool; 8] = [false; 8];
pub static mut VA_OPPONENT_X : [f32; 8] = [0.0; 8];
pub static mut VA_OPPONENT_Y : [f32; 8] = [0.0; 8];
pub static mut VA_OPPONENT_BOMA  : [u64; 8] = [0; 8];
static mut YOU_X : [f32; 8] = [0.0; 8];
static mut YOU_Y : [f32; 8] = [0.0; 8];
static mut VANISH_TIMER : [f32; 8] = [0.0; 8];
pub static mut ACTIVATE_VANISH : [bool; 8] = [false; 8];
pub static mut DISABLE_CATCH : [bool; 8] = [false; 8];
pub static mut DEFENDER_VANISH : [bool; 8] = [false; 8];
static mut VA_OPPONENT_DIRECTION_Y : [f32; 8] = [12.0; 8];
static mut VA_OPPONENT_DIRECTION_X : [f32; 8] = [12.0; 8];
static mut VANISH_DIREC : [i32; 8] = [0; 8];
pub static mut FLOAT_TIMER : [i32; 8] = [0; 8];
pub static mut WINDOW : [i32; 8] = [0; 8];
pub static mut WHO_GOT_HIT_BOMA : [u32; 8] = [0; 8];
pub static mut WHO_HIT_YOU_BOMA : [u32; 8] = [0; 8];
pub static mut PROJECTILE_BOMA : [u32; 8] = [0; 8];
//pub static mut attack_vanish_get_current_position : [bool; 8] = [false; 8];
pub static mut VANISH_HEIGHT : [f32; 8] = [0.0; 8];
pub static mut FLOAT : [bool; 8] = [false; 8];
static mut VERT_EXTRA : [f32; 8] = [6.0; 8];
static mut FLASH_TIMER : [i16; 8] = [-1; 8]; 
pub static mut ENABLE_CANCEL_INTO_VANISH : [bool; 8] = [false; 8];
//static mut EFFECTS_OFF : [bool; 8] = [false; 8];
//pub static mut VANISH_TO_ENTRY_ID : [u32; 8] = [0; 8];

pub mod opff;
pub mod resets_falses;

pub fn install() {
    opff::install();
    resets_falses::install();
}
