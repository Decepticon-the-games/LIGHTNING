use super::*;
#[acmd_script( agent = "szerosuit", script = "game_aircatchlanding", category = ACMD_GAME, low_priority )]
unsafe fn game_aircatchlanding(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.4);
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP2, *WEAPON_SZEROSUIT_WHIP2_STATUS_KIND_REWIND);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP2, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}