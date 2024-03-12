use super::*;
#[acmd_script( agent = "murabito", script = "game_attackdash", category = ACMD_GAME, low_priority )]
unsafe fn game_attackdash(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_FLOWERPOT, false, -1);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_FLOWERPOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        whiff_cancel(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_attackdash);
}