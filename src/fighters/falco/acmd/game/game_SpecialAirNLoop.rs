use super::*;
use crate::fighters::falco::opff::FASTFALL_LASER;
#[acmd_script( agent = "falco", script = "game_specialairnloop", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairnloop(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        FASTFALL_LASER[entry_id] = true;
        whiff_cancel(fighter);
    }
    if macros::IS_EXIST_ARTICLE(fighter, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER) {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, Hash40::new("loop"), false, -1.0);
        }
    }
    frame(fighter.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(fighter, 0.6);
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialairnloop);
}