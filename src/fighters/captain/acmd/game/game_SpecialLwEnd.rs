use super::*;
#[acmd_script( agent = "captain", script = "game_speciallwend", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    
            whiff_cancel(fighter);
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.05);
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_SP_BRAKE);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_SP_BRAKE);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
     game_speciallwend);
}