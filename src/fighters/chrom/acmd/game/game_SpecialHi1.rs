use super::*;
#[acmd_script( agent = "chrom", script = "game_specialhi1", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhi1(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.7);
    frame(fighter.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(fighter, 0.3);
}