use super::*;
#[acmd_script( agent = "gamewatch", script = "game_specialscommona", category = ACMD_GAME, low_priority )]
unsafe fn game_specialscommona(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_FLAG_FIX_PANEL);
        VisibilityModule::set_default_int64(fighter.module_accessor, Hash40::new("panel"));
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_ );
}