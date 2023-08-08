use super::*;
#[acmd_script( agent = "dolly", script = "game_specialairsfend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairsfend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
CANCEL_IN_NEUTRAL[entry_id] = true;
    frame(fighter.lua_state_agent, 0.0);
    if (WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W) {
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
    }
    else{
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 0.8);
        }
    } 
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairsfend);
}