use super::*;
#[acmd_script( agent = "palutena", script = "game_specialairhi", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairhi(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_DIVE);
        whiff_cancel(fighter);

    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_CONTROL_ON);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairhi);
}