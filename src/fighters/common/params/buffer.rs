use super::*;

pub unsafe fn common_precede_int_param(param_type: u64, param_hash: u64) -> Option<i32> {

    if param_hash == hash40("precede") { //1 buffer during neutral at all until comboing (buffer 10 on attacks hit)
        
        return Some(1);

    }
    if param_hash == hash40("precede_extension") {//NO HOLD BUFFER
        return Some(0);
    }
    None
}
