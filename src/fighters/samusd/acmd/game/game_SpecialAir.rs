use super::*;
#[acmd_script( agent = "samusd", script = "game_specialair", category = ACMD_GAME, low_priority )]
unsafe fn game_specialair(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_AIR_CONTROL);
    }
    whiff_cancel(fighter);
}    
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialair);
 }