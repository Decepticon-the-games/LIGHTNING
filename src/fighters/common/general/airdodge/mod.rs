use {
    smash::{
        lua2cpp::{L2CAgentBase,L2CFighterCommon},
        phx::Hash40,
        hash40,
        app::{lua_bind::*, sv_animcmd::*,*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

pub mod effect;
pub mod params;

pub fn install() {
    effect::install(); 
    //params::install(); 
}