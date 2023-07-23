use super::*;
use super::*;
#[acmd_script( agent = "murabito", script = "game_specialnfailure", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnfailure(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

CANCEL_IN_NEUTRAL[entry_id] = true;
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialnfailure);
}