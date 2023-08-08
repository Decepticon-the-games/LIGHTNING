use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*,*},
        lib::lua_const::*,
        hash40
    },
    smash_script::*,
    smashline::*
};

use crate::fighters::common::mechanics::{
    cancels::{
        attack_cancels::{ENABLE_ATTACK_CANCEL},
    },
    lightning_mechanics::{
      lightning_mode::{LIGHTNING, LIGHTNING_BUTTON},
    vanish::{VANISH_COUNT, VANISH_COUNTER, VANISH_BUTTON, CANCEL_INTO_VANISH},
    ultrainstinct::{CROSS_CANCEL_BUTTON},
    crimson_cancel::{CRIMSON_CANCEL_BUTTON},
    lightning_fsmeter::{FINAL_SMASH_BUTTON},  
    } 
};

//static mut MOTION_CHECK : [i32; 8] = [0; 8]; // Gets status kind while jump_guard_dash_upspecial_pressed. This is to avoid spam when u have no jumps/dodges left, so the status being checked would be the status being spammed. If it tdetects jump/dodge, it'll do nothing.
pub static mut CANCEL_IN_NEUTRAL : [bool; 8] = [false; 8];
pub static mut AIRDASH : [bool; 8] = [false; 8];
pub static mut WAVEDASH : [bool; 8] = [false; 8];
pub static mut WAVESTEP : [bool; 8] = [false; 8];
pub static mut AIRDODGE_PLUS : [bool; 8] = [false; 8];
pub static mut AIRSTEP : [bool; 8] = [false; 8];
static mut AIRDODGE_BUTTON : [bool; 8] = [false; 8];// for only running the code within it 1 frame.
static mut AIRDODGE_COUNT : [i32; 8] = [0; 8]; //  You start off with one airdodge. Every other airdodge after that before touching the ground increases the number up to how many jumps that fighter has.
pub static mut PROJECTILE_COUNTER : [bool; 8] = [false; 8];
pub static mut PROJECTILE_COUNT : [i32; 8] = [0; 8];
pub static mut DISABLE_MOVESET_TRANSITIONS : [bool; 8] = [false; 8];

pub mod airdash;
pub mod cancel_in_neutral;
pub mod motion_cancels;
pub mod multiple_airdodges;
pub mod resets_falses;
//pub mod transitions;
//pub mod wavestep;

pub fn install() {
    airdash::install();
    cancel_in_neutral::install();
    motion_cancels::install();
    multiple_airdodges::install();
    resets_falses::install();
    //wavestep::install();
}