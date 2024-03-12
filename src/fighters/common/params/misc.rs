use super::*;


pub unsafe fn common_misc_int_param(param_type: u64, param_hash: u64) -> Option<i32> {

    if param_hash == hash40("dash_speed_keep_frame") {
        return Some(1);
    }
    if param_hash == hash40("turn_dash_frame") {
        return Some(-1);
    }
    None
}

pub unsafe fn common_misc_float_param(param_type: u64, param_hash: u64) -> Option<f32> {

    if param_hash == hash40("invalid_passive_speed") {//remove knockback untechables
        return Some(999.0);
    }
    if param_hash == hash40("damage_fly_correction_max") {//Maximum changed angle from trajectory DI (degrees), just less than 2x
        return Some(18.0);
    }
    None
}
