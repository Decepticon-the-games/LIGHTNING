use super::*;
#[acmd_script( agent = "link", script = "game_specials1", category = ACMD_GAME, low_priority )]
unsafe fn game_specials1(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG, false, -1);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);        whiff_cancel(fighter);
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specials1);
 }