use super::*;
#[acmd_script( agent = "shizue", script = "game_speciallwset", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwset(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    macros::FT_MOTION_RATE(fighter, 1.208);
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_LW_FLAG_SET);
        whiff_cancel(fighter);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_speciallwset);
}