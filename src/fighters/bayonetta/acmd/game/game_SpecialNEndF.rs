use super::*;
#[acmd_script( agent = "bayonetta", script = "game_specialnendf", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnendf(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.48);
    whiff_cancel(fighter);
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialnendf);
}