use super::*;
#[acmd_script( agent = "szerosuit", script = "game_speciallwkicklanding", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwkicklanding(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
        whiff_cancel(fighter);
    }
    macros::FT_MOTION_RATE(fighter, 1.2);
}