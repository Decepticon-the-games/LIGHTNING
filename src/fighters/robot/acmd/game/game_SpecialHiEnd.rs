use super::*;
#[acmd_script( agent = "robot", script = "game_specialhiend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhiend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

whiff_cancel(fighter); 
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialhiend);
}