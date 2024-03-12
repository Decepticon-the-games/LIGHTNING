use super::*;
#[acmd_script( agent = "brave", script = "game_specialn3", category = ACMD_GAME, low_priority )]
unsafe fn game_specialn3(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 9.0, 6.0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_SUCCESS_SP) {
        if macros::is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_FIREBALL, false, -1);
            whiff_cancel(fighter);
        }
    }
    frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4.0, 4.0);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialn3);
}