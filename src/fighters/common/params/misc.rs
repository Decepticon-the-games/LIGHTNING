use super::*;

use crate::fighters::common::function_hooks::float_int_hook::{PARAM_INT_OFFSET, PARAM_FLOAT_OFFSET};

#[skyline::hook(offset = PARAM_INT_OFFSET)]
pub unsafe fn common_misc_int_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);


    if param_hash == hash40("dash_speed_keep_frame") {
        return 1;
    }
    if param_hash == hash40("turn_dash_frame") {
        return -1;
    }
    if param_hash == hash40("invalid_capture_frame") { //can't grab for 120 frames at a time
        return 120;
    }
    ret
}

#[skyline::hook(offset = PARAM_FLOAT_OFFSET)]
pub unsafe fn common_misc_float_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);

    if param_hash == hash40("invalid_passive_speed") {//remove knockback untechables
        return 999.0;
    }
    if param_hash == hash40("damage_fly_correction_max") {//Maximum changed angle from trajectory DI (degrees), just less than 2x


        return 18.0;
    }
    if param_hash == hash40("attack_hit_slow_rate") {//The total amount of animation frame passed during hitlag


        return 18.0;
    }
    if param_hash == hash40("escape_air_slide_landing_speed_max") {//airdodge landing speed (momentum transferred to slide on wavedash)
        return 18.0;
    }
    ret
}


pub fn install() {
    skyline::install_hooks!(
        common_misc_int_param_accessor_hook,
        common_misc_float_param_accessor_hook
    );
}