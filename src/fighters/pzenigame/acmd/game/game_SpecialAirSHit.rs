use super::*;
#[acmd_script( agent = "pzenigame", script = "game_specialairshit", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairshit(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.9);
}