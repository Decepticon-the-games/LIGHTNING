use super::*;
#[acmd_script( agent = "krool", script = "game_attacks3lw", category = ACMD_GAME, low_priority )]
unsafe fn game_attacks3lw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
        ENABLE_ATTACK_CANCEL[entry_id] = true; 
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 361, 76, 0, 64, 5.0, 0.0, 11.0, 11.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("havel"), 13.0, 361, 76, 0, 64, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
CANCEL_IN_NEUTRAL[entry_id] = true;
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_attacks3lw);
}