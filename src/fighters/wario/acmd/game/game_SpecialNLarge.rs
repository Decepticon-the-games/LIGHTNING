use super::*;
#[acmd_script( agent = "wario", script = "game_specialnlarge", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnlarge(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_N_FLAG_ITEM_REMOVE);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_N_FLAG_ITEM_USE);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_ );
}