use super::*;

//REMOVE SDI 

pub unsafe fn remove_sdi_param_int(param_type: u64, param_hash: u64) -> Option<i32> {
    
    if param_hash == hash40("hit_stop_delay_flick") {
        
        return Some(0);
    }
    None
}
