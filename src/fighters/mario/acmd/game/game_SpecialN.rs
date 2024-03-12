use super::*;
#[acmd_script( agent = "mario", script = "game_specialn", category = ACMD_GAME, low_priority )]
unsafe fn game_specialn(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    macros::FT_MOTION_RATE(fighter, 1.15);
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, -1);
        whiff_cancel(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialn);
}