use super::*;
use crate::fighters::common::mechanics::cancels::counter_cancels::COUNTER_CANCEL;
#[acmd_script( agent = "littlemac", script = "game_specialairlwhit", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlwhit(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

COUNTER_CANCEL[entry_id] = true;

    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ATTACK_START);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        ENABLE_ATTACK_CANCEL[entry_id] = true; 
macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.0, 361, 50, 0, 90, 6.0, 3.0, 0.0, 0.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 50, 0, 90, 10.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(9.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
CANCEL_IN_NEUTRAL[entry_id] = true;
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ATTACK_END);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_LW_FLAG_COUNTER_ENABLE_GRAVITY);
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
        game_specialairlwhit);
}