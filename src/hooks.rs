
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use crate::lightning_01_ultrainstinct::SECRET_SENSATION;
use crate::lightning_01_upbtransitions::UP_SPECIAL;

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term )]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let ret = original!()(module_accessor,term);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    
    if SECRET_SENSATION[entry_id] {
        return false;
    }
    if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL {
        return false;
    }
    if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI {
        if UP_SPECIAL[entry_id] {
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
pub fn install() {
    skyline::install_hook!(is_enable_transition_term_replace);
}