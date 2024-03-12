use super::*;
#[acmd_script( agent = "snake", script = "game_specialairlwground", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlwground(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        whiff_cancel(fighter);
    }  
}  
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairlwground );
}