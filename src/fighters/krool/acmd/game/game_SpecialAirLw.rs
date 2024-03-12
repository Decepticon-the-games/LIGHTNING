use super::*;
#[acmd_script( agent = "krool", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KROOL_STATUS_SPECIAL_LW_FLAG_SHIELD);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 43.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    frame(fighter.lua_state_agent, 68.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 82.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairlw);
}