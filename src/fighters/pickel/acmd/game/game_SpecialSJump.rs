use super::*;
#[acmd_script( agent = "pickel", script = "game_specialsjump", category = ACMD_GAME, low_priority )]
unsafe fn game_specialsjump(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PICKEL_STATUS_SPECIAL_S_FLAG_CHANGE_STATUS_FALL);
    }
}