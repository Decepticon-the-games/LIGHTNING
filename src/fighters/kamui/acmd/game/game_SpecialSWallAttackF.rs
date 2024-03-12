use super::*;
#[acmd_script( agent = "kamui", script = "game_specialswallattackf", category = ACMD_GAME, low_priority )]
unsafe fn game_specialswallattackf(fighter: &mut L2CAgentBase) {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40::new("special_s_wall_attack_f"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 55, 98, 0, 70, 8.3, 0.0, 2.0, 5.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_MOVE_KINETIC_PARAM);
    }
    wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 55, 90, 0, 70, 4.5, 0.0, 3.7, 8.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_S_FLAG_AIR_CONTROL);
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialswallattackf);
}