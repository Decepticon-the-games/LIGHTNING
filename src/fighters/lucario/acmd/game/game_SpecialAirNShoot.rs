use super::*;
#[acmd_script( agent = "lucario", script = "game_specialairnshoot", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairnshoot(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
whiff_cancel(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairnshoot);
}