use super::*;
#[acmd_script( agent = "wiifit", script = "game_specialhiend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhiend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        CANCEL_IN_NEUTRAL[entry_id] = true;
ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_HULAHOOP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialhiend);
}