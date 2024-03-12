use super::*;
#[acmd_script( agent = "peach", script = "game_specialairsend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairsend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_S_JUMP_FLAG_START_CONTROLLER_MOVE);
    whiff_cancel(fighter);
}
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialairsend);
}