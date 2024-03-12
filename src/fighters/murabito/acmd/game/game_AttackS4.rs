use super::*;
#[acmd_script( agent = "murabito", script = "game_attacks4", category = ACMD_GAME, low_priority )]
unsafe fn game_attacks4(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_BOWLING_BALL, false, -1);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_BOWLING_BALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        whiff_cancel(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_attacks4);
}