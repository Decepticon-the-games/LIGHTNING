use super::*;
#[acmd_script( agent = "miiswordsman", script = "game_specialairlw1hit", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlw1hit(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_OFF);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        ENABLE_ATTACK_CANCEL[entry_id] = true; 
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 73, 60, 0, 85, 10.5, 0.0, 7.0, 14.0, Some(0.0), Some(7.0), Some(4.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
CANCEL_IN_NEUTRAL[entry_id] = true;
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_GRAVITY_ON);
    }
    macros::FT_MOTION_RATE(fighter, 0.6);
}  
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairlw1hit);
}