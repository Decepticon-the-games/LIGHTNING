use super::*;
#[acmd_script( agent = "ike", script = "game_specialhi4", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhi4(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ENABLE_ATTACK_CANCEL[entry_id] = true;
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 50, 155, 0, 70, 10.0, 0.0, 6.0, 11.8, Some(0.0), Some(11.0), Some(11.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
CANCEL_IN_NEUTRAL[entry_id] = true;
    }  
}  
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialhi4 );
}