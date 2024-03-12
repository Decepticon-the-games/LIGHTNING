use super::*;
#[acmd_script( agent = "zelda", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 40.0);
    macros::FT_MOTION_RATE(fighter, 0.9);
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_speciallw);
}