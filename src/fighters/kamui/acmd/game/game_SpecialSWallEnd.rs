use super::*;
#[acmd_script( agent = "kamui_spearhand", script = "game_specialswallend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialswallend(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        if entry_id < 8 {
            whiff_cancel(fighter); 
        }
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialswallend);
}