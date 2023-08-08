use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::hash40; 
use skyline::hooks::{getRegionAddress, Region};
use smash::app::FighterManager;

use crate::fighters::common::mechanics::lightning_mechanics::ultrainstinct::{SECRET_SENSATION, CROSS_CANCEL_BUTTON};
use crate::fighters::common::mechanics::lightning_mechanics::vanish::{VANISH, VANISH_BUTTON, DISABLE_CATCH};
use crate::fighters::common::mechanics::misc::upbtransitions::DISABLE_UP_SPECIAL;
use crate::fighters::common::mechanics::lightning_mechanics::lightning_fsmeter::{DISABLE_FINAL, FINAL_SMASH_BUTTON};
use crate::fighters::common::mechanics::lightning_mechanics::crimson_cancel::{CRIMSON_CANCEL_BUTTON, CRIMSON_CANCEL_TIMER};
use crate::fighters::common::mechanics::lightning_mechanics::lightning_mode::{LIGHTNING_BUTTON};
use crate::fighters::common::mechanics::misc::misc::{TRANSITION_TERM_NONE};
use crate::fighters::common::mechanics::cancels::motioncancels::DISABLE_MOVESET_TRANSITIONS;

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term )]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let ret = original!()(module_accessor,term);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let special_mechanics_button = (CRIMSON_CANCEL_BUTTON[entry_id]
    || CROSS_CANCEL_BUTTON[entry_id]
    || VANISH_BUTTON[entry_id]
    || LIGHTNING_BUTTON[entry_id]);

    let moveset_out_of_odash_term = (
        //Jabs
        //term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK
        //|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100
        term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH
        //Tilts
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3
        //|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3
        //Aerials
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR
        //Smash Attacks
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_HOLD
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_HOLD
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_HOLD
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
        //|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START
        //Specials
        //|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI
        //|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
        //Command Attacks
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB
    );
    
    if SECRET_SENSATION[entry_id] {
        return false;
    }
    else if VANISH[entry_id] {
        return false;
    }
    else if TRANSITION_TERM_NONE[entry_id] {
        return false;
    }
    else if DISABLE_UP_SPECIAL[entry_id] {
        if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI {
            return false;
        }
        else {
            return ret;
        }
    }
    else if DISABLE_FINAL[entry_id] {
        if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL {
            return false;
        }
        else {
            return ret;
        }
    }
    else if DISABLE_MOVESET_TRANSITIONS[entry_id] {
        if  moveset_out_of_odash_term {
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