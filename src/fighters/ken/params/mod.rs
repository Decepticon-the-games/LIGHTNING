use super::*;

use crate::fighters::common::function_hooks::float_int_hook::{FIGHTER_PARAM_INT_OFFSET, FIGHTER_PARAM_FLOAT_OFFSET, INT_SEARCH_CODE, FLOAT_SEARCH_CODE};

use super::*;

#[skyline::hook(offset = FIGHTER_PARAM_INT_OFFSET)]
pub unsafe fn name_int_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1  as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);

    if fighter_kind == FIGHTER_KIND_/**/ {
        if param_type == hash40("/**/") {
            if param_hash == hash40("/**/") {
                
                return /**/ ;
            } 
        }          
    }
    ret
}
#[skyline::hook(offset = FIGHTER_PARAM_FLOAT_OFFSET)]
pub unsafe fn name_float_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1  as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);

    if fighter_kind == FIGHTER_KIND_/**/ {
        if param_type == hash40("/**/") {
            if param_hash == hash40("/**/") {
                
                return /**/ ;
            } 
        }          
    }
    ret
}
pub fn install() {
    skyline::install_hooks!(
        name_int_param_accessor_hook,
        name_float_param_accessor_hook
    );
}