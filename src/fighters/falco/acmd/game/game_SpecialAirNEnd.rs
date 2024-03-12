use super::*;
#[acmd_script( agent = "falco", script = "game_specialairnend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairnend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.75);
    if macros::IS_EXIST_ARTICLE(fighter, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER) {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, Hash40::new("close"), false, -1.0);
            whiff_cancel(fighter);
        }
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialairnend);
}