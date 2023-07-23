use super::*;
#[acmd_script( agent = "miigunner", script = "game_speciallw1end", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallw1end(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

CANCEL_IN_NEUTRAL[entry_id] = true;
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_speciallw1end);
}