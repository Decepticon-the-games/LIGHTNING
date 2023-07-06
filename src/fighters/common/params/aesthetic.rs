use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        hash40,
        app::{lua_bind::*, sv_animcmd::*,*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

use crate::fighters::common::function_hooks::float_int_hook::{PARAM_INT_OFFSET, PARAM_FLOAT_OFFSET};
#[skyline::hook(offset = PARAM_FLOAT_OFFSET)]
pub unsafe fn aesthetic_float_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);

    if param_hash == hash40("hitstop_frame_max") {//Maximum hitlag frames
        return 30 as f32;
    }
    if param_hash == hash40("hitstop_frame_add") {// Base hitlag frames incease, except jabs of any kind
        if (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        || (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK) {
            return 6 as f32;
        }
        else {
            return 12 as f32;
        }
    }
    if param_hash == hash40("hitstop_frame_mul") {//Multiplier to damage for hitlag
        return  0.4;
    }
    ret
}

pub fn install() {
    skyline::install_hooks!(
        //aesthetic_float_param_accessor_hook
    );
}