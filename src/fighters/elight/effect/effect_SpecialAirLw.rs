use super::*;
#[acmd_script( agent = "elight", script = "effect_specialairlw", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialairlw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("elight_change_start"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1.3, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_DETACH_KIND(fighter, Hash40::new("elight_change_start"), -1);
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(effect_specialairlw);
}