use super::*;


use crate::fighters::common::function_hooks::float_int_hook::{PARAM_INT_OFFSET, PARAM_FLOAT_OFFSET};

#[skyline::hook(offset = PARAM_FLOAT_OFFSET)]
pub unsafe fn fs_meter_float_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);

    if param_hash == hash40("charge_final_attack_reaction_mul") {
        
        return 0.7;
    } 
    ret
}
pub fn install() {
    skyline::install_hooks!(
        //fs_meter_float_param_accessor_hook
    );
}