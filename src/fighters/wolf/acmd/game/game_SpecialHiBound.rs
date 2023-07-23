use super::*;
#[acmd_script( agent = "wolf", script = "game_specialhibound", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhibound(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::SET_AIR(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialhibound);
}