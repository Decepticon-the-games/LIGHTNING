use super::*;
#[acmd_script( agent = "diddy", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_ITEM_BANANA, false, -1);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_LW_FLAG_ITEM_THROW);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_ITEM_BANANA, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_speciallw );
}
