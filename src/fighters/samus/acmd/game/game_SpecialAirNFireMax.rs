use super::*;
#[acmd_script( agent = "samus", script = "game_specialairnfiremax", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairnfiremax(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT);
        whiff_cancel(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialairnfiremax);
 }