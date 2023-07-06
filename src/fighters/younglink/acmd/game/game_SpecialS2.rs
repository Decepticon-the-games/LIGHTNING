use super::*;
#[acmd_script( agent = "younglink", script = "game_specials2", category = ACMD_GAME, low_priority )]
unsafe fn game_specials2(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_BOOMERANG, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specials2);
}