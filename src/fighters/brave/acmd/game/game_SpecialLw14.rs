use super::*;
#[acmd_script( agent = "brave", script = "game_speciallw14", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallw14(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        CANCEL_IN_NEUTRAL[entry_id] = true;
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_SLEEP, false, -1);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLAG_ENABLE_CONTROL_ENERGY);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_speciallw14);
}