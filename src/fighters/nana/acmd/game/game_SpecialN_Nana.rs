use super::*;
#[acmd_script( agent = "nana", script = "game_specialn_nana", category = ACMD_GAME, low_priority )]
unsafe fn game_specialn_nana(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_ICESHOT, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_ENABLE_COUPLE);
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialn_nana);
}