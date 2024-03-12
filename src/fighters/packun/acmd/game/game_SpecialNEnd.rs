use super::*;
#[acmd_script( agent = "packun", script = "game_specialnend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PACKUN_GENERATE_ARTICLE_SPIKEBALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        whiff_cancel(fighter);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialnend);
}