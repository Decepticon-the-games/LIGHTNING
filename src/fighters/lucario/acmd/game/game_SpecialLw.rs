use super::*;
#[acmd_script( agent = "lucario", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, *FIGHTER_LUCARIO_SHIELD_KIND_SPLIT, *FIGHTER_LUCARIO_SHIELD_GROUP_KIND_SPLIT);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, *FIGHTER_LUCARIO_SHIELD_KIND_SPLIT, *FIGHTER_LUCARIO_SHIELD_GROUP_KIND_SPLIT);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
    frame(fighter.lua_state_agent, 35.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_speciallw);
}