use super::*;

pub static mut FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_VANISH : [bool; 8] = [false; 8]; //FAKE transition_term for vanish
pub static mut VANISH_BUTTON : [bool; 8] = [false; 8]; //Only used to replicate a button_pad_trigger, runs only 1 frame
pub static mut VANISH_BUFFER : [bool; 8] = [false; 8];
pub static mut CAN_VANISH : [bool; 8] = [false; 8];//Incorporating the ability to use vanish under condition. See lightning_01_motioncancels/mod.rs

pub static mut FIGHTER_STATUS_KIND_VANISH : [bool; 8] = [false; 8];
pub static mut VANISH_COUNTER : [bool; 8] = [false; 8];
pub static mut VANISH_COUNT : [i32; 8] = [0; 8];//See motioncancels/mod.rs
pub static mut VANISH_RESET : [bool; 8] = [false; 8];//See motioncancels/mod.rs
static mut CAMERA : [bool; 8] = [false; 8];

//everything to do with opponent information
pub static mut VA_OPPONENT_X : [f32; 8] = [0.0; 8];
pub static mut VA_OPPONENT_Y : [f32; 8] = [0.0; 8];
pub static mut VA_OPPONENT_BOMA  : [u64; 8] = [0; 8];
static mut VA_OPPONENT_DIRECTION_Y : [f32; 8] = [12.0; 8];
static mut VA_OPPONENT_DIRECTION_X : [f32; 8] = [12.0; 8];
pub static mut WHO_GOT_HIT_BOMA : [u32; 8] = [0; 8];
pub static mut WHO_HIT_YOU_BOMA : [u32; 8] = [0; 8];
pub static mut KB_SPEED : [f32; 8] = [0.0; 8];

static mut YOU_X : [f32; 8] = [0.0; 8];
static mut YOU_Y : [f32; 8] = [0.0; 8];
static mut VANISH_TIMER : [f32; 8] = [0.0; 8];
pub static mut DEFENDER_VANISH : [bool; 8] = [false; 8];

//pub static mut VANISH_HEIGHT : [f32; 8] = [WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0); 8]; //Gets every character's specific height for vanish position
pub static mut VERT_EXTRA : [f32; 8] = [12.0; 8];

pub static mut PROJECTILE_BOMA : [u32; 8] = [0; 8];
pub static mut FLOAT : [bool; 8] = [false; 8];
pub static mut FLOAT_TIMER : [i32; 8] = [0; 8];
pub static mut WINDOW : [i32; 8] = [0; 8];


pub mod opff;
pub mod resets_falses;

pub fn install() {
    opff::install();
    resets_falses::install();
}
