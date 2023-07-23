use super::*;
#[acmd_script( agent = "rockman", script = "game_specialairnturn", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairnturn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 7.0);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_ALREADY_EXIST_METALBLADE) {
        if macros::is_excute(fighter) {
            ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_METALBLADE, false, -1);
        }
    }
    frame(fighter.lua_state_agent, 18.0);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_ALREADY_EXIST_METALBLADE) {
        if macros::is_excute(fighter) {
            ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_METALBLADE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
            CANCEL_IN_NEUTRAL[entry_id] = true;
        }
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairnturn);
}