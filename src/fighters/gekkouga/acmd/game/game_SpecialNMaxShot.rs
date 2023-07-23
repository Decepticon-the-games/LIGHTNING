use super::*;
#[acmd_script( agent = "gekkouga", script = "game_specialnmaxshot", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnmaxshot(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_N_WORK_FLAG_SHURIKEN_SHOOT);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialnmaxshot);
}