use super::*;
#[acmd_script(agent = "simon", script = "game_specialairn", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, false, -1);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SIMON_GENERATE_ARTICLE_AXE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairn);
}