use super::*;
#[acmd_script( agent = "demon", script = "game_specialairn", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 2.0);
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, false, -1);
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        whiff_cancel(fighter);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 43.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 46.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 50.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 54.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 56.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 7.0);
    frame(fighter.lua_state_agent, 58.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 60.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairn);
}