use super::*;
#[acmd_script( agent = "falco", script = "game_specialsend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialsend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialsend);
}