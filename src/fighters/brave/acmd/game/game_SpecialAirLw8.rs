use super::*;
#[acmd_script( agent = "brave", script = "game_specialairlw8", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlw8(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_CRASH, false, -1);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_FULLBURST_INTERRUPT);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 116.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_FULLBURST_INTERRUPT);
    }
    frame(fighter.lua_state_agent, 136.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_ENABLE_GRAVITY);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairlw8);
}