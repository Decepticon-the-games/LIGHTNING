use super::*;
#[acmd_script( agent = "reflet", script = "game_specialairlwend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlwend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    whiff_cancel(fighter);
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 18.0);
    macros::FT_MOTION_RATE(fighter, 0.7);
    frame(fighter.lua_state_agent, 38.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}
    pub fn install() {
        smashline::install_acmd_scripts!(
        game_specialairlwend);
    }