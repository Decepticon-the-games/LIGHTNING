use super::*;
#[acmd_script( agent = "edge", script = "game_specialairn2", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairn2(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.8);
    frame(fighter.lua_state_agent, 11.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE, false, -1);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    }
    frame(fighter.lua_state_agent, 35.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    frame(fighter.lua_state_agent, 60.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairn2);
}