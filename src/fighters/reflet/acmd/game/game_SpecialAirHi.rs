use super::*;
#[acmd_script( agent = "reflet", script = "game_specialairhi", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairhi(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, -1);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
        CANCEL_IN_NEUTRAL[entry_id] = false;
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairhi);
}