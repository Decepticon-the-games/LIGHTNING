use super::*;
#[acmd_script( agent = "donkey", script = "game_speciallwstart", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.75);
}