use super::*;
#[acmd_script( agent = "dolly", script = "game_speciallwend", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 1, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        whiff_cancel(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_speciallwend);
}