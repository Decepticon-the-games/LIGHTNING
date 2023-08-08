use super::*;
#[acmd_script( agent = "dolly", script = "game_specialsfend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialsfend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
CANCEL_IN_NEUTRAL[entry_id] = true;
    if (WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 1.2);
        }
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialsfend);
}