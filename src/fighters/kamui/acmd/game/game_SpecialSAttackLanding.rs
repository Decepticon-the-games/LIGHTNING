use super::*;
#[acmd_script( agent = "kamui", script = "game_specialsattacklanding", category = ACMD_GAME, low_priority )]
unsafe fn game_specialsattacklanding(fighter: &mut L2CAgentBase) {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
        whiff_cancel(fighter);
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialsattacklanding);
}