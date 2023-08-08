use super::*;
#[acmd_script(agent = "simon", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn game_attacklw3(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        //ENABLE_ATTACK_CANCEL[entry_id] = true; 
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 5.0, 40, 20, 0, 65, 3.0, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 5.0, 40, 20, 0, 55, 2.8, -1.5, 0.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        if AttackModule::is_attack_occur(fighter.module_accessor) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        if ! AttackModule::is_attack_occur(fighter.module_accessor) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        CANCEL_IN_NEUTRAL[entry_id] = true;
        //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_attacklw3);
}