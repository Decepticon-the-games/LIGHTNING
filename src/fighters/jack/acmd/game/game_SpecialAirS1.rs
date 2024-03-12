use super::*;
#[acmd_script( agent = "jack", script = "game_specialairs1", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairs1(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_FIRE, false, -1);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_ENABLE_CONTROL_ENERGY);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_SET_FALL_NORMAL);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairs1);
}
 