use super::*;
#[acmd_script( agent = "elight", script = "game_speciallwend", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
CancelModule::enable_cancel(fighter.module_accessor);

}
pub fn install() {
    smashline::install_acmd_scripts!(
    game_speciallwend);
}