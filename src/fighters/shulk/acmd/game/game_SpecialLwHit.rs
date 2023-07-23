use super::*;
use crate::fighters::common::mechanics::cancels::counter_cancels::COUNTER_CANCEL;

#[acmd_script( agent = "shulk", script = "game_speciallwhit", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwhit(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    COUNTER_CANCEL[entry_id] = true; 

}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_speciallwhit);
}