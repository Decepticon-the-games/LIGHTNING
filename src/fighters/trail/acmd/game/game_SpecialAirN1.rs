use super::*;
#[acmd_script( agent = "trail", script = "game_specialairn1", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairn1(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TRAIL_GENERATE_ARTICLE_FIRE, false, -1);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_N1_FLAG_CAN_INPUT);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_N1_FLAG_CHANGE_MAGIC);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
    frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_N1_FLAG_CAN_INPUT);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairn1);
}