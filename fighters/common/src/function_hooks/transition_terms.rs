use super::*;

use crate::mechanics::ultrainstinct::{SECRET_SENSATION, CROSS_CANCEL_BUTTON};
use crate::mechanics::vanish::{VANISH, VANISH_BUTTON};
use crate::mechanics::upbtransitions::DISABLE_UP_SPECIAL;
use crate::mechanics::lightning_fsmeter::{DISABLE_FINAL, FINAL_SMASH_BUTTON};
use crate::mechanics::crimson_cancel::{CRIMSON_CANCEL_BUTTON, CRIMSON_CANCEL_TIMER};
use crate::mechanics::lightning_mode::{LIGHTNING_BUTTON};

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term )]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let ret = original!()(module_accessor,term);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let special_mechanics_button = (CRIMSON_CANCEL_BUTTON[entry_id]
    || CROSS_CANCEL_BUTTON[entry_id]
    || VANISH_BUTTON[entry_id]
    || LIGHTNING_BUTTON[entry_id]);

    
    if SECRET_SENSATION[entry_id] {
        return false;
    }
    if VANISH[entry_id] {
        return false;
    }
    //if special_mechanics_button 
    //{
    //    return false;
    //}


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

pub fn install() {

    skyline::install_hook!(is_enable_transition_term_replace);

}