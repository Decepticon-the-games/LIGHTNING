use super::*;

pub unsafe fn parries_dyn_int_param(param_type: u64, param_hash: u64) -> Option<i32> {

    if param_hash == hash40("shield_just_frame") {// parry window
                
        return Some(15);
    }
    if param_hash == hash40("continue_shield_just_count") {// parry consecutively by hit count
                
        return Some(40);
    } 
    None
}

pub unsafe fn parries_dyn_float_param(param_type: u64, param_hash: u64) -> Option<f32> {

    if param_hash == hash40("0x2cacbf2a63") {// Frame advantage after parrying????
                
        return Some(30.0);
    }
    None
}