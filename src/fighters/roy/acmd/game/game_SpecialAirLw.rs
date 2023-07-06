use super::*;
#[acmd_script( agent = "roy", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_SHIELD);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_APPLY_POWERUP_MOTION_RATE);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_SHIELD);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_APPLY_POWERUP_MOTION_RATE);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairlw );
}