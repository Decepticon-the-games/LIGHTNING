use super::*;
#[acmd_script( agent = "inkling", script = "game_specialairlwmin", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlwmin(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SPLASHBOMB, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INKLING_STATUS_SPECIAL_LW_FLAG_THROW);
        whiff_cancel(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairlwmin);
}