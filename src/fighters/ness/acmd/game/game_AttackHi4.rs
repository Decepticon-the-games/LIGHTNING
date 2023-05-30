use super::*;
#[acmd_script( agent = "ness", script = "game_attackhi4", category = ACMD_GAME, low_priority )]
unsafe fn game_attackhi4(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.74);
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);    }
    frame(fighter.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 56.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_YOYO_HEAD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_attackhi4);
}