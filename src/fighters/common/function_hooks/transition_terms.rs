use super::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::hash40; 
use skyline::hooks::{getRegionAddress, Region};
use smash::app::FighterManager;

use crate::fighters::common::mechanics::lightning_mechanics::ultrainstinct::{FIGHTER_STATUS_KIND_CROSS_CANCEL};
use crate::fighters::common::mechanics::misc::upbtransitions::DISABLE_UP_SPECIAL;

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term )]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let ret = original!()(module_accessor,term);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(module_accessor);


    let combo_flags = (WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    || WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_INPUT)
    || WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE)
    || WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
    || WorkModule::is_flag(module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO)
    || WorkModule::is_flag(module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_ENABLE_COMBO)
    || WorkModule::is_flag(module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO)
    || WorkModule::is_flag(module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_ENABLE_COMBO)
    || WorkModule::is_flag(module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_COMBO));

    if FIGHTER_STATUS_KIND_CROSS_CANCEL[entry_id] {
        return false;
    }
    else if FIGHTER_STATUS_KIND_VANISH[entry_id] {
        return false;
    }
    else if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL {
        return false;
    }   
    else if CANCEL_IN_NEUTRAL[entry_id] {
        if ! (term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL
        )
        {
            return false;
        }
        else {
            return ret;
        }
    }
    else if (status_kind == *FIGHTER_STATUS_KIND_ATTACK 
        || status_kind == *FIGHTER_TANTAN_STATUS_KIND_ATTACK_COMBO) 
        && AttackModule::is_attack_occur(module_accessor) {
        if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK /*&& ! combo_flags*/ {
            return false;
        }
        else {
            return ret;
        }
    }
    else if DISABLE_UP_SPECIAL[entry_id] {
        if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI {
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