use super::*;
#[acmd_script( agent = "donkey", script = "game_specialairnstart", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairnstart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_ );
}