use super::*;
#[acmd_script( agent = "popo", script = "game_specialhithrow2nana_nana", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhithrow2nana_nana(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_HI_CLIFF_PULL_PARTNER_FLAG_PULL);
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_);
}