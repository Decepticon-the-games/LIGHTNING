use super::*;

use crate::fighters::common::function_hooks::float_int_hook::{FIGHTER_PARAM_INT_OFFSET, FIGHTER_PARAM_FLOAT_OFFSET, INT_SEARCH_CODE, FLOAT_SEARCH_CODE};
/*
#[skyline::hook(offset = FIGHTER_PARAM_INT_OFFSET)]
pub unsafe fn shulk_int_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);

    if fighter_kind == FIGHTER_KIND_SHULK {
        if param_type == hash40("/**/") {
            if param_hash == hash40("/**/") {
                
                return /**/;
            } 
        }          
    }
    ret
}
*/
#[skyline::hook(offset = FIGHTER_PARAM_FLOAT_OFFSET)]
pub unsafe fn shulk_float_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);

    if fighter_kind == FIGHTER_KIND_SHULK {
        if param_type == hash40("param_special_n") {//3x Longer lasting monado, shorter cooldown
            if param_hash == hash40("active_time_speed") { 
                return 24.0;
            }
            if param_hash == hash40("active_time_shield") { 
                return 18.0;
            }
            if param_hash == hash40("active_time_buster") { 
                return 30.0;
            }
            if param_hash == hash40("active_time_smash") { 
                return 16.0;
            }
            if param_hash == hash40("unavailable_time_jump") { 
                return 8.0;
            }
            if param_hash == hash40("unavailable_time_speed") { 
                return 8.0;
            }
            if param_hash == hash40("unavailable_time_shield") { 
                return 8.0;
            } 
            if param_hash == hash40("unavailable_time_buster") { 
                return 8.0;
            } 
            if param_hash == hash40("unavailable_time_smash") { 
                return 8.0;
            } 
        }   
        if param_type == hash40("param_special_lw") {
            if param_hash == hash40("slow_mul") { 
                return 20.0;
            }
        }       
    }
    ret
}
pub fn install() {
    skyline::install_hooks!(
        //shulk_int_param_accessor_hook,
        shulk_float_param_accessor_hook
    );
}