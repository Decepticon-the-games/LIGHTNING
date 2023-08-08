use super::*;
#[acmd_script( agent = "brave", script = "game_specialairs2", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairs2(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_S_FLAG_ENABLE_SPARK);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_S_FLAG_SUCCESS_SP) {
        if macros::is_excute(fighter) {
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 90, 0, 25, 4.5, 0.0, 11.5, 12.0, Some(0.0), Some(11.5), Some(56.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_MAGIC);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 10, 4);
        }
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        if AttackModule::is_attack_occur(fighter.module_accessor) {
            ENABLE_ATTACK_CANCEL[entry_id] = true; 
        }  
        else {
            CANCEL_IN_NEUTRAL[entry_id] = true;
        }  
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairs2);
}