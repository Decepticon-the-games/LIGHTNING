use super::*;

pub unsafe fn grabs_dyn_float_param(param_type: u64, param_hash: u64) -> Option<f32> {

    if param_hash == hash40("invalid_capture_frame") { //can't grab for 120 frames at a time
        return Some(120.0);
    }
    None
}