use super::*;
#[acmd_script( agent = "zelda", script = "game_specialsend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialsend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_S_FLAG_2);
    }
}
pub fn install() {
smashline::install_acmd_scripts!(
 game_specialsend);
}