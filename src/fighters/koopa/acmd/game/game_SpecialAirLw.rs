use super::*;
#[acmd_script( agent = "koopa", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG1);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 290, 65, 0, 13, 8.6, 0.0, 5.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        enable_attack_cancel(fighter); 
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 76, 75, 0, 45, 8.3, 0.0, 5.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        enable_attack_cancel(fighter); 
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialairlw);
}