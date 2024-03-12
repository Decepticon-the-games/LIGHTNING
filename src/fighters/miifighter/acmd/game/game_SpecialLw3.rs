use super::*;
#[acmd_script( agent = "miifighter", script = "game_speciallw3", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallw3(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_SHIELD);
    }
    frame(fighter.lua_state_agent, 8.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_SHIELD);
        whiff_cancel(fighter);
    }
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(fighter, 0.4);
    frame(fighter.lua_state_agent, 40.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 60.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}   
pub fn install() {
    smashline::install_acmd_scripts!(
    game_speciallw3);
}