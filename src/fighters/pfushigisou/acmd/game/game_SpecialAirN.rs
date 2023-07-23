use super::*;
#[acmd_script( agent = "pfushigisou", script = "game_specialairn", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    wait_loop_clear(fighter.lua_state_agent);
    frame(fighter.lua_state_agent, 1.0);
}