use super::*;
#[acmd_script( agent = "trail", script = "game_speciallwrebound", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwrebound(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
COUNTER_CANCEL[entry_id] = true; 
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_speciallwrebound);
}