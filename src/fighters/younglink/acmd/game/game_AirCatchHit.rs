use super::*;
#[acmd_script( agent = "younglink", script = "game_aircatchhit", category = ACMD_GAME, low_priority )]
unsafe fn game_aircatchhit(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_aircatchhit );
}