use super::*;
#[acmd_script( agent = "dolly", script = "game_specialhifall", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhifall(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        CANCEL_IN_NEUTRAL[entry_id] = true;
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialhifall);
}