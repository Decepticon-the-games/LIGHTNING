use super::*;
#[acmd_script( agent = "master", script = "game_specialn", category = ACMD_GAME, low_priority )]
unsafe fn game_specialn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_ARROW1, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialn);
}