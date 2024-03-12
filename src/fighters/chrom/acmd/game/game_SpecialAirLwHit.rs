use super::*;
use crate::fighters::common::mechanics::cancels::counter_cancels::COUNTER_CANCEL;
#[acmd_script( agent = "chrom", script = "game_specialairlwhit", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlwhit(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    COUNTER_CANCEL[entry_id] = true;
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        COUNTER_CANCEL[entry_id] = false;
        enable_attack_cancel(fighter); 
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 95, 0, 45, 7.7, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(3.0), 1.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 361, 65, 0, 35, 7.5, 0.0, 8.0, 18.0, Some(0.0), Some(8.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairlwhit );
}