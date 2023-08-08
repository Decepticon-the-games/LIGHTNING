use super::*;
#[acmd_script( agent = "tantan", script = "game_specialairhiend2", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairhiend2(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 3, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 9, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 10, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 11, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 12, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 13, *HIT_STATUS_NORMAL);
        macros::HIT_NO(fighter, 19, *HIT_STATUS_NORMAL);
    }
    frame(fighter.lua_state_agent, 3.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TANTAN_STATUS_SPECIAL_HI_FLAG_IS_BLOCKED) {
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
CANCEL_IN_NEUTRAL[entry_id] = true;
        }
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairhiend2);
}