use super::*;
#[acmd_script( agent = "reflet", script = "game_speciallwend", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    whiff_cancel(fighter);
    frame(fighter.lua_state_agent, 18.0);
    macros::FT_MOTION_RATE(fighter, 0.7);
    frame(fighter.lua_state_agent, 38.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_speciallwend);
}