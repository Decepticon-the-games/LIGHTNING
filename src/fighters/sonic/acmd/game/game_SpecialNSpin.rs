use super::*;
#[acmd_script( agent = "sonic", script = "game_specialnspin", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnspin(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 50);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_);
}