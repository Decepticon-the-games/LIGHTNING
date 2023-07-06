use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*,*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

pub mod upbtransitions;
pub mod edgecancelling;
pub mod moonwalking;
pub mod misc;

pub fn install() {
    upbtransitions::install();
    edgecancelling::install();
    moonwalking::install();
    misc::install();
}

