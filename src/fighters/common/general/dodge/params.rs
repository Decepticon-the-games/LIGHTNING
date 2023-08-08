use super::*;

use crate::fighters::common::function_hooks::float_int_hook::{PARAM_INT_OFFSET, PARAM_FLOAT_OFFSET};

//remove_dodge_penalty
#[skyline::hook(offset = PARAM_INT_OFFSET)]
pub unsafe fn remove_dodge_penalty_int_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);

    if param_hash == hash40("escae_penalty_frame") {
        return 1;
    }
    if param_hash == hash40("escae_penalty_recovery_frame") {
        return 1;
    }
    ret
}
#[skyline::hook(offset = PARAM_FLOAT_OFFSET)]
pub unsafe fn remove_dodge_penalty_float_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);

    if param_hash == hash40("escae_b_penalty_motion_rate") {
        return 1.0;
    }
    ret
}
pub fn install() {
    skyline::install_hooks!(
        //remove_dodge_penalty_int_param_accessor_hook,
        //remove_dodge_penalty_float_param_accessor_hook
    );
}