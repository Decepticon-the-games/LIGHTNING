use {
    smash::{
        lua2cpp::{L2CFighterCommon,L2CFighterBase,L2CAgentBase},
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*,BattleObjectModuleAccessor,*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

use crate::fighters::common::{
    mechanics::{
        lightning_mechanics::vanish::{VANISH_READY, CANCEL_INTO_VANISH},
        cancels::motioncancels::CANCEL_IN_NEUTRAL
    },
    function_hooks::cross_cancel_vanish::PROJECTILE_HIT
};

pub static mut ATTACK_CANCEL : [bool; 8] = [false; 8];
pub static mut ENABLE_ATTACK_CANCEL : [bool; 8] = [false; 8];
pub static mut ATTACK_CANCEL_COUNTER : [bool; 8] = [false; 8];
pub static mut ATTACK_CANCEL_COUNT : [i32; 8] = [0; 8];
pub static mut ENABLE_MULTIHIT_CANCEL : [bool; 8] = [false; 8];
pub static mut MOVEMENT_CANCEL : [bool; 8] = [true; 8];

pub mod attack_cancel;
pub mod spam_system;
pub mod resets_falses;

pub fn install() {
    attack_cancel::install();
    //spam_system::install();
    resets_falses::install();
}