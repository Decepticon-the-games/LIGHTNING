use super::*;
#[acmd_script( agent = "buddy", script = "game_specialnfire", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnfire(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_GENERATE_BULLET);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_ENABLE_SHOOT);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_);
}