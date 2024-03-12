use super::*;
#[acmd_script( agent = "koopajr", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.25);
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_MECHAKOOPA, false, -1);
        whiff_cancel(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairlw);
}