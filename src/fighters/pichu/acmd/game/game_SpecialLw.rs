use super::*;
#[acmd_script( agent = "pichu", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_KAMINARI_GENERATE);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_speciallw);
}