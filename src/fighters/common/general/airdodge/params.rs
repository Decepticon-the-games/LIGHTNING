use super::*;

pub unsafe fn airdodge_int_param(param_type: u64, param_hash: u64) -> Option<i32> {

    if param_hash == hash40("escape_air_slide_back_end_frame") {
        return Some(0);
    }
    None
}

pub unsafe fn airdodge_dyn_float_param(param_type: u64, param_hash: u64) -> Option<f32> {

    if param_type == hash40("param_motion") {

        if param_hash == hash40("escape_air_slide_back_distance") {
            return Some(0.0);
        }
        if param_hash == hash40("escape_air_slide_speed") {
            return Some(8.0);
        }
        if param_hash == hash40("escape_air_slide_distance") {
            return Some(30.0);
        }
        if param_hash == hash40("escape_air_slide_penalty_speed") {
            return Some(8.0);
        }
        if param_hash == hash40("escape_air_slide_penalty_distance") {
            return Some(30.0);
        }
        if param_hash == hash40("escape_air_slide_end_speed") {
            return Some(0.4);
        }
        if param_hash == hash40("landing_speed_mul_escape_air_slide") {
            return Some(1.0);
        }
    }
    if param_hash == hash40("escape_air_slide_landing_speed_max") {//airdodge landing speed (momentum transferred to slide on wavedash)
        return Some(18.0);
    }
    None
}