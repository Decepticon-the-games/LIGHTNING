use super::*;
#[acmd_script( agent = "szerosuit", script = "game_speciallwlanding", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwlanding(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
        whiff_cancel(fighter);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SZEROSUIT_STATUS_SPECIAL_LW_FLAG_TREAD_ENABLE);
    }
}