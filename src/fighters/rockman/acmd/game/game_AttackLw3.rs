use super::*;
#[acmd_script( agent = "rockman", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn game_attacklw3(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        ENABLE_ATTACK_CANCEL[entry_id] = true; 
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 70, 60, 0, 92, 3.5, 0.0, 3.0, 3.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 50, 56, 0, 80, 3.5, 0.0, 3.0, 3.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
        AttackModule::clear_all(fighter.module_accessor);
CANCEL_IN_NEUTRAL[entry_id] = true;
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_attacklw3);
}