use super::*;
#[acmd_script( agent = "pfushigisou", script = "game_specialairs", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairs(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.25);
    frame(fighter.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_LEAFCUTTER, false, -1);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 29.0);
    macros::FT_MOTION_RATE(fighter, 0.9);
    frame(fighter.lua_state_agent, 49.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}  
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairs );
}
