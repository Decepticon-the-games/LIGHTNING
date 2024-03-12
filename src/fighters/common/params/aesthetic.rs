use super::*;

pub unsafe fn aesthetic_float_param(param_type: u64, param_hash: u64) -> Option<f32> {

    if param_hash == hash40("hitstop_frame_max") {//Maximum hitlag frames
        return Some(30.0);
    }
    if param_hash == hash40("hitstop_frame_add") {// Base hitlag frames incease, except jabs of any kind
        //if (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        //|| (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        //|| (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK) {
        //    return Some(6.0);
        //}
        //else {
            return Some(12.0);
        //}
    }
    if param_hash == hash40("hitstop_frame_mul") {//Multiplier to damage for hitlag
        return Some(0.4);
    }
    if param_hash == hash40("attack_hit_slow_rate") {//The total amount of animation frame passed during hitlag
        return Some(18.0);
    }
    None
}
