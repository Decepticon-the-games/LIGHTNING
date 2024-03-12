use super::*;

//remove_dodge_penalty

pub unsafe fn remove_dodge_penalty_int_param(param_type: u64, param_hash: u64) -> Option<i32> {

    if param_hash == hash40("escae_penalty_frame") {
        return Some(1);
    }
    if param_hash == hash40("escae_penalty_recovery_frame") {
        return Some(1);
    }
    None
}

pub unsafe fn remove_dodge_penalty_float_param(param_type: u64, param_hash: u64) -> Option<f32> {

    if param_hash == hash40("escae_b_penalty_motion_rate") {
        return Some(0.06);
    }
    None
}
