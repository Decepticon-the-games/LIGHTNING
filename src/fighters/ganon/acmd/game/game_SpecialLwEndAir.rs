use super::*;
#[acmd_script( agent = "ganon", script = "game_speciallwendair", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwendair(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_FALL_ONOFF);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_CLIFF_CHECK);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
   game_speciallwendair );
}