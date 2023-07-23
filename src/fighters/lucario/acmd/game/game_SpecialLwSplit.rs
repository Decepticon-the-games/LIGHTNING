use super::*;
use crate::fighters::common::mechanics::cancels::counter_cancels::COUNTER_CANCEL;
#[acmd_script( agent = "lucario", script = "game_speciallwsplit", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwsplit(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
COUNTER_CANCEL[entry_id] = true;
    if macros::is_excute(fighter) {
        
        macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_speciallwsplit);
}