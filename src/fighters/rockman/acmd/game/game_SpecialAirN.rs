use super::*;
#[acmd_script( agent = "rockman", script = "game_specialairn", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 7.0);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_ALREADY_EXIST_METALBLADE) {
        if macros::is_excute(fighter) {
            ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_METALBLADE, false, -1);
        }
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_STICK_CHECK_FRAME_END);
    }
    frame(fighter.lua_state_agent, 16.0);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_ALREADY_EXIST_METALBLADE) {
        if macros::is_excute(fighter) {
            ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_METALBLADE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
            whiff_cancel(fighter);
        }
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairn);
}