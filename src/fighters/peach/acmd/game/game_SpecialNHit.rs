use super::*;
use crate::fighters::common::mechanics::counter_cancels::COUNTER_CANCEL;
#[acmd_script( agent = "peach", script = "game_specialnhit", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnhit(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        COUNTER_CANCEL[entry_id] = true;
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIOSPORE, false, -1);
        COUNTER_CANCEL[entry_id] = false;
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIOSPORE, false, -1);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIOSPORE, false, -1);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIOSPORE, false, -1);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIOSPORE, false, -1);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIOSPORE, false, -1);
    }
    frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialnhit);
}