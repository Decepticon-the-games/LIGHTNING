use super::*;
#[acmd_script( agent = "lucas", script = "game_specials", category = ACMD_GAME, low_priority )]
unsafe fn game_specials(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: -0.9, y: 0.1, z: 0.0});
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FIRE, false, -1);
        whiff_cancel(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specials);
}