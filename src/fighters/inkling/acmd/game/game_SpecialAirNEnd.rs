use super::*;
#[acmd_script( agent = "inkling", script = "game_specialairnend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairnend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
whiff_cancel(fighter);
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.65);
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairnend);
}