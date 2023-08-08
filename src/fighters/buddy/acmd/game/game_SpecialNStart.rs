use super::*;
#[acmd_script( agent = "buddy", script = "game_specialnstart", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnstart(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    0x18cc20(*FIGHTER_INSTANCE_WORK_ID_INT_KIND, *FIGHTER_KIND_KIRBY);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_BUDDY_GENERATE_ARTICLE_PARTNER, false, -1);
    }
    macros::FT_MOTION_RATE(fighter, 0.75);
    wait(fighter.lua_state_agent, 36.0);
    macros::FT_MOTION_RATE(fighter, 0.25);
    frame(fighter.lua_state_agent, 40.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}