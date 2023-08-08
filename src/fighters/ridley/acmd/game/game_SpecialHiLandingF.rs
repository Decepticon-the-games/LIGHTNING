use super::*;
#[acmd_script( agent = "ridley", script = "game_specialhilandingf", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhilandingf(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
CANCEL_IN_NEUTRAL[entry_id] = true;
    if macros::is_excute(fighter) {
        macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialhilandingf);
}