use super::*;
#[acmd_script( agent = "szerosuit", script = "game_specialnshoot", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnshoot(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_N_FLAG_SHOOT);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_PARALYZER_BULLET, false, -1);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialnshoot );
}