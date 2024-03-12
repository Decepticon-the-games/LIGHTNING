use super::*;
#[acmd_script( agent = "inkling", script = "game_specialhilanding", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhilanding(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, true);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SPLASH, false, -1);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialhilanding);
}