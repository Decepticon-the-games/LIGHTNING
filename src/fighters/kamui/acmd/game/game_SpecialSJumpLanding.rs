use super::*;
#[acmd_script( agent = "kamui", script = "game_specialsjumplanding", category = ACMD_GAME, low_priority )]
unsafe fn game_specialsjumplanding(fighter: &mut L2CAgentBase) {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40::new("special_s_jump_landing"), false, -1.0);
        CANCEL_IN_NEUTRAL[entry_id] = true;

    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialsjumplanding);
}