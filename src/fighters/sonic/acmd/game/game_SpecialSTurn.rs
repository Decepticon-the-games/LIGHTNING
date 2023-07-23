use super::*;
#[acmd_script( agent = "sonic", script = "game_specialsturn", category = ACMD_GAME, low_priority )]
unsafe fn game_specialsturn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_);
}