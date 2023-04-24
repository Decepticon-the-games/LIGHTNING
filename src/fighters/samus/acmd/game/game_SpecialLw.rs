use super::*;
#[acmd_script( agent = "samus", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_WEAPON);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CROUCH);
    }
    macros::FT_MOTION_RATE(fighter, 0.6);
}    
pub fn install() {
    smashline::install_acmd_scripts!(
     game_speciallw);
 }