use super::*;
#[acmd_script( agent = "sonic", script = "game_specialnrebound", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnrebound(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.15);
}