use super::*;
#[acmd_script( agent = "robot", script = "game_specialairhi", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairhi(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROBOT_STATUS_BURNER_FLAG_TRANSFORM_COMP);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairhi);
}