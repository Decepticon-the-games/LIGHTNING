use super::*;
#[acmd_script( agent = "miigunner", script = "game_specialn2end", category = ACMD_GAME, low_priority )]
unsafe fn game_specialn2end(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIIGUNNER_RAPID_SHOT_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
        whiff_cancel(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialn2end);
}