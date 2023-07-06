use super::*;
#[acmd_script( agent = "pitb", script = "game_specialairsend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairsend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        ENABLE_ATTACK_CANCEL[entry_id] = true; 
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 40, 58, 0, 90, 6.2, 5.0, 4.0, 9.0, Some(5.0), Some(10.0), Some(9.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: -0.5, y: 2.5, z: 0.0});
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
CANCEL_IN_NEUTRAL[entry_id] = true;
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
    frame(fighter.lua_state_agent, 22.0);
    macros::FT_MOTION_RATE(fighter, 0.83);
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairsend );
}