use super::*;
#[acmd_script( agent = "pikmin", script = "game_specials", category = ACMD_GAME, low_priority )]
unsafe fn game_specials(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_SPECIAL_S_FLAG_IS_HOLD) {
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_SPECIAL_S_FLAG_THROW);
CANCEL_IN_NEUTRAL[entry_id] = true;
        }
        else{
        frame(fighter.lua_state_agent, 378992935.0);
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 0.6);
        }
    }
}    
}
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specials);
}