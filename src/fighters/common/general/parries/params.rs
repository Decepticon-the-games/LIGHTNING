use super::*;


use crate::fighters::common::function_hooks::float_int_hook::{PARAM_INT_OFFSET, PARAM_FLOAT_OFFSET};

#[skyline::hook(offset = PARAM_INT_OFFSET)]
pub unsafe fn parries_int_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);

    if param_hash == hash40("shield_just_frame") {// parry window
                
        return 15;
    }
    if param_hash == hash40("continue_shield_just_count") {// parry consecutively by hit count
                
        return 40;
    } 
    ret
}

#[skyline::hook(offset = PARAM_FLOAT_OFFSET)]
pub unsafe fn parries_float_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);

    if param_hash == hash40("0x2cacbf2a63") {// Frame advantage after parrying????
                
        return 30.0;
    }
    ret
}

pub fn install() {
    skyline::install_hooks!(
        //parries_int_param_accessor_hook,
        //parries_float_param_accessor_hook
    );
}