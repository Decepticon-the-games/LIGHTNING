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
use crate::fighters::common::function_hooks::float_int_hook::{PARAM_INT_OFFSET, PARAM_FLOAT_OFFSET};

//REMOVE SDI 

#[skyline::hook(offset = 0x4E53C0)]
pub unsafe fn get_param_int_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    //pub use crate::fighters::cloud::params::limit_never_runs_out;
    
    if param_hash == hash40("hit_stop_delay_flick") {
        
        return 0x0;
    }
    ret;
}
pub fn install() {
    skyline::install_hook!(
        get_param_int_replace
    );
}