use super::*;
#[acmd_script( agent = "gamewatch", script = "game_attackairhi", category = ACMD_GAME, low_priority )]
unsafe fn game_attackairhi(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BREATH, false, -1);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_BREATH, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);    }
    frame(fighter.lua_state_agent, 41.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_attackairhi );
}