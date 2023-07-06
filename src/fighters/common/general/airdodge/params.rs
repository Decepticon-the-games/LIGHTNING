use super::*;


use crate::fighters::common::function_hooks::float_int_hook::{PARAM_INT_OFFSET, PARAM_FLOAT_OFFSET};

#[skyline::hook(offset = PARAM_INT_OFFSET)]
pub unsafe fn airdodge_int_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);

    if param_hash == hash40("escape_air_slide_back_end_frame") {
        return 0;
    }
    ret
}

#[skyline::hook(offset = PARAM_FLOAT_OFFSET)]
pub unsafe fn airdodge_float_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);

    if param_hash == hash40("escape_air_slide_back_distance") {
        return 0.0;
    }
    if param_hash == hash40("escape_air_slide_speed") {
        return  8.0;
    }
    if param_hash == hash40("escape_air_slide_distance") {
        return  30.0;
    }
    if param_hash == hash40("escape_air_slide_penalty_speed") {
        return  8.0;
    }
    if param_hash == hash40("escape_air_slide_penalty_distance") {
        return  30.0;
    }
    if param_hash == hash40("escape_air_slide_end_speed") {
        return  0.4;
    }
    if param_hash == hash40("landing_speed_mul_escape_air_slide") {
        return  1.0;
    }
    ret
}
pub fn install() {
    skyline::install_hooks!(
        //airdodge_int_param_accessor_hook,
        //airdodge_float_param_accessor_hook
    );
}