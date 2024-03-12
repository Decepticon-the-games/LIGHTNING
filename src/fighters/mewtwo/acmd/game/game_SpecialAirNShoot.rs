use super::*;
#[acmd_script( agent = "mewtwo", script = "game_specialairnshoot", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairnshoot(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    frame(fighter.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_MEWTWO_GENERATE_ARTICLE_SHADOWBALL, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
        whiff_cancel(fighter);
    }
}
pub fn install() {
smashline::install_acmd_scripts!(
   game_specialairnshoot );
}