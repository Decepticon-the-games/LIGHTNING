use super::*;
#[acmd_script( agent = "brave", script = "game_specialn1", category = ACMD_GAME, low_priority )]
unsafe fn game_specialn1(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 10.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_SUCCESS_SP) {
        if macros::is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_FIREBALL, false, -1);
            whiff_cancel(fighter);
        }
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialn1);
}