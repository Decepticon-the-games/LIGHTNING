use super::*;
#[acmd_script( agent = "miiswordsman", script = "game_specialairn2", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairn2(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_LIGHTSHURIKEN, false, -1);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_LIGHTSHURIKEN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairn2);
}