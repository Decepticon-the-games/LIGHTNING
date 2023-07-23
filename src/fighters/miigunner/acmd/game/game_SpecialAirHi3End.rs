use super::*;
#[acmd_script( agent = "miigunner", script = "game_specialairhi3end", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairhi3end(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    CANCEL_IN_NEUTRAL[entry_id] = true;
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairhi3end);
}