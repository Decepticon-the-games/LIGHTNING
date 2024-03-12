use super::*;
#[acmd_script( agent = "popo", script = "game_specialairhithrow", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairhithrow(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_HI_FLAG_FAIL_CLIFF_START);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_HI_JUMP_PRE_FLAG_ADVANCE_JUMP);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        whiff_cancel(fighter);
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialairhithrow);
}