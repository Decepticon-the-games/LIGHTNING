use super::*;

use crate::fighters::common::function_hooks::float_int_hook::{FIGHTER_PARAM_INT_OFFSET, FIGHTER_PARAM_FLOAT_OFFSET, INT_SEARCH_CODE, FLOAT_SEARCH_CODE};

#[skyline::hook(offset = FIGHTER_PARAM_INT_OFFSET)]
pub unsafe fn cloud_int_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);

    if fighter_kind == FIGHTER_KIND_CLOUD {
        if param_type == hash40("param_special_lw") && param_hash == hash40("limit_break_clear_frame") {
            
            return 999999; //limit_never_runs_out
        }          
    }
    ret
}
#[skyline::hook(offset = FIGHTER_PARAM_FLOAT_OFFSET)]
pub unsafe fn cloud_float_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);

    if fighter_kind == FIGHTER_KIND_CLOUD {
        if param_type == hash40("param_special_lw") && param_hash == hash40("limit_gauge_add") {
            
            return 0.6; //faster limit charge
        }          
    }
    ret
}
pub fn install() {
    skyline::install_hooks!(
        cloud_float_param_accessor_hook,
        cloud_int_param_accessor_hook
    );
}