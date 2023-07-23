use super::*;
#[acmd_script( agent = "rockman", script = "game_speciallwshoot", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwshoot(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_LW_SHOOT_WORK_ID_FLAG_REQUEST_SHOOT);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_speciallwshoot);
}