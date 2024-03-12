use super::*;
#[acmd_script( agent = "pickel", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn game_attacklw3(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    macros::FT_MOTION_RATE(fighter, 8.0);
    frame(fighter.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FIRE, false, -1);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
}
pub fn install() {
    smashline::install_acmd_scripts!(
    game_attacklw3);
}