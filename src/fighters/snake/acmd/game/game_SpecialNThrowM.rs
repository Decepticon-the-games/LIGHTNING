#[acmd_script( agent = "snake", script = "game_specialnthrowm", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnthrowm(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    battle_object();
    methodlib::L2CValue::L2CValue(void*)();
    methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
    methodlib::L2CValue::as_pointer()const(*FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, *ARTICLE_OPE_TARGET_LAST);
    is_constraint_article();
    if(false){
        if macros::is_excute(fighter) {
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, *ARTICLE_OPE_TARGET_LAST, false);
        }
    }
}