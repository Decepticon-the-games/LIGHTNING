use super::*;
#[acmd_script( agent = "tantan", script = "game_specialhishortend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhishortend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 2, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 3, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 4, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 5, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 6, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 7, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 8, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 9, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 10, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 11, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 12, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 13, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 18, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 19, *HIT_STATUS_OFF);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 2, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 3, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 4, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 5, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 6, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 7, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 8, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 9, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 10, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 11, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 12, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 13, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 18, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 19, *HIT_STATUS_NORMAL);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialhishortend);
}