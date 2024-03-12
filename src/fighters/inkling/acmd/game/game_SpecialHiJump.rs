use super::*;
#[acmd_script( agent = "inkling", script = "game_specialhijump", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhijump(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SPLASH, false, -1);
    }
    frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        whiff_cancel(fighter);
    }
    
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialhijump);
}