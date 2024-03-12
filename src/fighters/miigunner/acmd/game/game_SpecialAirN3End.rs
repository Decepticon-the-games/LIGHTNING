use super::*;
#[acmd_script( agent = "miigunner", script = "game_specialairn3end", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairn3end(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GRENADELAUNCHER, false, -1);
        whiff_cancel(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairn3end);
}