use super::*;
#[acmd_script( agent = "samus", script = "game_specialairs", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairs(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_AIR_CONTROL);
    }
    frame(fighter.lua_state_agent, 58.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
}    
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialairs);
 }