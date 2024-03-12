use super::*;
#[acmd_script( agent = "pikachu", script = "game_specialsend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialsend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        JostleModule::set_status(fighter.module_accessor, true);
        whiff_cancel(fighter);
    }
} 
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialsend);
}