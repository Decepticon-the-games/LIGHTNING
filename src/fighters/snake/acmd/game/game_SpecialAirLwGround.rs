#[acmd_script( agent = "snake", script = "game_specialairlwground", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlwground(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *ARTICLE_OPE_TARGET_ALL, false);
    }
}