
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::hash40; 
use crate::lightning_01_ultrainstinct2::SECRET_SENSATION;
use crate::lightning_01_upbtransitions::DISABLE_UP_SPECIAL;
use crate::lightning_01_lightning_fsmeter::DISABLE_FINAL;

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term )]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let ret = original!()(module_accessor,term);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    
    if SECRET_SENSATION[entry_id] {
        return false;
    }
    
    if DISABLE_UP_SPECIAL[entry_id] {
        if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI {
            return false;
        }
        else {
            return ret;
        }
    }
    if DISABLE_FINAL[entry_id] {
        if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL {
            return false;
        }
        else {
            return ret;
        }
    }
    else {
        return ret;
    }


}

#[skyline::hook(offset = 0x4E53C0)]
pub unsafe fn get_param_int_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let _fighter_kind = utility::get_kind(boma);
    if param_hash == hash40("hit_stop_delay_flick") {
        if FighterUtil::is_hp_mode(boma) {
            return 0x4;
        }
    }
    return ret;
}

#[skyline::hook(offset = 0x4E5380)]
pub unsafe fn get_param_float_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let _fighter_kind = utility::get_kind(boma);
    if param_hash == hash40("hit_stop_delay_stick") {
        if FighterUtil::is_hp_mode(boma) {
            return 0.7;
        }
    }
    return ret;
    
}
pub fn install() {
    skyline::install_hook!(is_enable_transition_term_replace);
    //skyline::install_hook!(get_param_float_replace);
    skyline::install_hook!(get_param_int_replace);
}