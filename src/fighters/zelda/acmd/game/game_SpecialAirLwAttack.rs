use super::*;
#[acmd_script( agent = "zelda", script = "game_specialairlwattack", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlwattack(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        whiff_cancel(fighter);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialairlwattack);
}