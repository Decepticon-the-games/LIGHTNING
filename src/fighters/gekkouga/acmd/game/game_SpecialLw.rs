use super::*;
#[acmd_script( agent = "gekkouga", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_LW_FLAG_SHIELD);
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_LW_FLAG_SHIELD);
        whiff_cancel(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_speciallw);
}