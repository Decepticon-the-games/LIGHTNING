use super::*;
use super::*;
#[acmd_script( agent = "murabito", script = "game_specialnfire", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnfire(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.6);
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_SPECIAL_N_FLAG_TAKEOUT);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
}