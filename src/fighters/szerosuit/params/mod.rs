use super::*;

use crate::fighters::common::function_hooks::float_int_hook::{FIGHTER_PARAM_INT_OFFSET, FIGHTER_PARAM_FLOAT_OFFSET, INT_SEARCH_CODE, FLOAT_SEARCH_CODE};

#[skyline::hook(offset = FIGHTER_PARAM_INT_OFFSET)]
pub unsafe fn zss_int_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);

    if fighter_kind == FIGHTER_KIND_SZEROSUIT {
        if param_type == hash40("param_paralyzer_bullet") {
            if param_hash == hash40("life") {
                
                return 30; //bullet life 30 frames
            } 
        }          
    }
    ret
}
#[skyline::hook(offset = FIGHTER_PARAM_FLOAT_OFFSET)]
pub unsafe fn zss_float_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;


    if fighter_kind == FIGHTER_KIND_SZEROSUIT {
        if param_type == hash40("param_paralyzer_bullet") {
            if param_hash == hash40("speed_tame") {
                if IS_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING[entry_id] {
                    return 5.8; //faster speed
                }
                else {
                    return 1.8; //faster speed
                }
                
            } 
        }          
    }
    ret
}
pub fn install() {
    skyline::install_hooks!(
        zss_int_param_accessor_hook,
        zss_float_param_accessor_hook
    );
}