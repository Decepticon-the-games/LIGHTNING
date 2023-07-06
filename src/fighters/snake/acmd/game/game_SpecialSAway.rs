#[acmd_script( agent = "snake", script = "game_specialsaway", category = ACMD_GAME, low_priority )]
unsafe fn game_specialsaway(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, true, *WEAPON_SNAKE_NIKITA_INSTANCE_WORK_ID_FLAG_AWAY);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}