use super::*;
#[acmd_script( agent = "pit", script = "game_specialsend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialsend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        ENABLE_ATTACK_CANCEL[entry_id] = true; 
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 80, 72, 0, 100, 6.0, 0.0, 4.0, 9.0, Some(0.0), Some(10.0), Some(9.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
CANCEL_IN_NEUTRAL[entry_id] = true;
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialsend );
}