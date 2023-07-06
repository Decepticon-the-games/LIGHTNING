use super::*;

use crate::fighters::common::function_hooks::float_int_hook::{PARAM_INT_OFFSET, PARAM_FLOAT_OFFSET};

#[skyline::hook(offset = PARAM_INT_OFFSET)]
pub unsafe fn shields_int_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);

    if param_hash == hash40("shield_setoff_catch_frame") {//Frames until shield grab is possible after shield stun
        return 7;
    }
    if param_hash == hash40("shield_setoff_escape") {//Number of consecutive hits during shieldstun to cancel with dodges
        return 5;
    }
    if param_hash == hash40("guard_off_cancel_frame") {//Move out of shield frame 1
        return 1;
    }
    if param_hash == hash40("guard_off_enable_shield_frame") {//Minimum time between shields (frames)
        return 30;
    }
    return ret;
}
#[skyline::hook(offset = 0x4E5380)]
pub unsafe fn shields_get_param_float_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);

    if param_hash == hash40("shield_max") {//2x lifespan (100)
       return 100 as f32;
    }
    
    if param_hash == hash40("shield_scale_min") {//Minimum shield size
        return 0.8;
    }
    if param_hash == hash40("shield_scale_mul") {//shield size multiplier
        return 1.0;
    }
    if param_hash == hash40("shield_setoff_add") {//Base shieldstun frames
        return 0.0;
    }
    if param_hash == hash40("shield_setoff_mul") {//Base shieldstun multiplier
        return 1.0;
    }
    if param_hash == hash40("shield_setoff_mul_fighter_shot") {//Shieldstun multiplier for projectile attacks
        return 1.0;
    }
    if param_hash == hash40("shield_stiff_mul_attack_4") {//Shieldstun multiplier for smash attacks
        return 1.0;
    }
    if param_hash == hash40("shield_stiff_mul_attack_air") {//Shieldstun multiplier for aerial attacks
        return 1.0;
    }
    if param_hash == hash40("shield_setoff_speed_mul") {//shield pushback multiplier
        return 0.1;
    }
    if param_hash == hash40("shield_setoff_speed_max") {//Maximum shield pushback is slightly raised
        return 1.9;
    }
    if param_hash == hash40("shield_pattern_power_mul") {//Stale factor multiplier with hitting shield, drastically decreased
        return 0.15;
    }
    if param_hash == hash40("shield_reset") {//life after shield break recovery (100)
        return 100.0;
    }
    if param_hash == hash40("shield_comp_mul") {//shield tilt speed normalized
        return 100.0;
    }
    if param_hash == hash40("shield_comp_reach_mul") {//shield tilt speed normalized
        return 100.0;
    }
    ret
}
pub fn install() {
    skyline::install_hooks!(
        //shields_int_param_accessor_hook,
        //shields_get_param_float_replace
    );
}
