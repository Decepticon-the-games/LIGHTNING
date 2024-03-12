use super::*;
#[acmd_script( agent = "miigunner", script = "game_specialairs2start", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairs2start(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIIGUNNER_STATUS_STEALTH_BOMB_FLAG_1);
        whiff_cancel(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairs2start);
}