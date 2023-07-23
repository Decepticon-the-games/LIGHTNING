use super::*;
use super::*;
#[acmd_script( agent = "koopajr", script = "game_specialhijrrise", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhijrrise(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_STATUS_SPECIAL_HI_SHOOT_FLAG_ACTION);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialhijrrise);
}