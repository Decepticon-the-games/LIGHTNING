use super::*;
#[acmd_script( agent = "purin", script = "game_specialnendl", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnendl(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
        JostleModule::set_status(fighter.module_accessor, true);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialnendl);
}