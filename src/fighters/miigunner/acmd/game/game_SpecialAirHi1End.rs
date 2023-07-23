use super::*;
#[acmd_script( agent = "miigunner", script = ["game_specialairhi1end", "0x172df1d06a", "0x14f601a966"], category = ACMD_GAME, low_priority )]
unsafe fn game_specialairhi1(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

CANCEL_IN_NEUTRAL[entry_id] = true;
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairhi1);
}