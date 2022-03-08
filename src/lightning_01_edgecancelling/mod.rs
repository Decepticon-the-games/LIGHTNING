use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::{self};
use smash::lua2cpp::{L2CFighterCommon};
use smash::lib::lua_const::*;
use smash::app::utility::*;
use smashline::*;

#[skyline::hook(replace = smash::app::lua_bind::StatusModule::init_settings)]
unsafe fn init_settings_replace(module_accessor: &mut app::BattleObjectModuleAccessor, situation_kind: i32, arg3: i32, arg4: u64, ground_cliff_check_kind: u64, arg6: bool, arg7: i32, arg8: i32, arg9: i32, arg10: i32) -> u64 {
    let status_kind = StatusModule::status_kind(module_accessor);
    let fighter_kind = app::utility::get_kind(module_accessor);
    if fighter_kind == FIGHTER_KIND_BUDDY 
    && (status_kind == *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S) 
    && situation_kind == SITUATION_KIND_GROUND {
        original!()(module_accessor, situation_kind, arg3, 7 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    }
    else if fighter_kind == FIGHTER_KIND_DONKEY
    && (status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH)
    && situation_kind == SITUATION_KIND_GROUND {
        original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    } 
    else if fighter_kind == FIGHTER_KIND_DIDDY
    && (status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH)
    && situation_kind == SITUATION_KIND_GROUND {
        original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    } 
    else if fighter_kind == FIGHTER_KIND_SONIC
    && (status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH)
    && situation_kind == SITUATION_KIND_GROUND {
        original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    } 
    else if status_kind == *FIGHTER_STATUS_KIND_APPEAL
    || status_kind == *FIGHTER_STATUS_KIND_DASH
    || status_kind == *FIGHTER_STATUS_KIND_TURN
    || status_kind == *FIGHTER_STATUS_KIND_TURN_DASH
    || status_kind == *FIGHTER_STATUS_KIND_LANDING
    || status_kind == *FIGHTER_STATUS_KIND_LANDING_LIGHT
    || status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR
    || status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL {
        original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    }
    else if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        if ControlModule::get_stick_y(module_accessor) >= 0.66 {
            original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
        }
        else {
            original!()(module_accessor, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)            
        }
    }
    else {
        original!()(module_accessor, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
    }
}

#[skyline::hook(replace = smash::app::lua_bind::GroundModule::correct)]
unsafe fn correct_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
ground_correct_kind: u32) -> u64 {
    let status_kind = StatusModule::status_kind(module_accessor);
    let fighter_kind = app::utility::get_kind(module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR 
    || status_kind == *FIGHTER_STATUS_KIND_LANDING
    || status_kind == *FIGHTER_STATUS_KIND_TURN_DASH 
    || status_kind == *FIGHTER_STATUS_KIND_DASH 
    || status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL {
        original!()(module_accessor, 1 as u32)
    }
    else if (fighter_kind == FIGHTER_KIND_PIKACHU && status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK)
    || (fighter_kind == FIGHTER_KIND_PIKACHU && status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK)
    || (fighter_kind == FIGHTER_KIND_PIKACHU && status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END)
    || (fighter_kind == FIGHTER_KIND_PICHU && status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK)
    || (fighter_kind == FIGHTER_KIND_PICHU && status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK)
    || (fighter_kind == FIGHTER_KIND_PICHU && status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END)
    || (fighter_kind == FIGHTER_KIND_FOX && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S)
    || (fighter_kind == FIGHTER_KIND_FALCO && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S)
    || (fighter_kind == FIGHTER_KIND_CAPTAIN && status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END)
    || (fighter_kind == FIGHTER_KIND_MIIFIGHTER && status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK_LANDING)
    || (fighter_kind == FIGHTER_KIND_DONKEY && status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH)
    || (fighter_kind == FIGHTER_KIND_DIDDY && status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH)
    || (fighter_kind == FIGHTER_KIND_SONIC && status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH) {
        original!()(module_accessor, *GROUND_CORRECT_KIND_GROUND as u32)
    }
    else {
        original!()(module_accessor, ground_correct_kind)
    }
}

pub fn install() {
    skyline::install_hook!(init_settings_replace);
    skyline::install_hook!(correct_replace);
}