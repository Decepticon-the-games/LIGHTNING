use super::*;
#[acmd_script( agent = "metaknight", script = "game_specialairlwend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlwend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        CANCEL_IN_NEUTRAL[entry_id] = true;
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("special_lw_end"), false, -1.0);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_METAKNIGHT_SPECIAL_LW_END);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairlwend );
}
