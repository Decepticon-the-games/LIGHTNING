use super::*;
#[acmd_script( agent = "miiswordsman", script = "game_specialairs1end", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairs1end(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
CANCEL_IN_NEUTRAL[entry_id] = true;
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_HENSOKU_SLASH_WORK_FLAG_END_LANDING);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairs1end);
}