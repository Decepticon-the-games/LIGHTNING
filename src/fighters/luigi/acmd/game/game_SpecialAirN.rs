use super::*;
#[acmd_script( agent = "luigi", script = "game_specialairn", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, -1);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialairn);
}