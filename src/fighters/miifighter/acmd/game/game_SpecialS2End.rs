use super::*;
#[acmd_script( agent = "miifighter", script = "game_specials2end", category = ACMD_GAME, low_priority )]
unsafe fn game_specials2end(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        CANCEL_IN_NEUTRAL[entry_id] = true;
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(fighter, 0.7);
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specials2end);
}