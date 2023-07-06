use super::*;
#[acmd_script( agent = "pichu", script = "game_specialairhiend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairhiend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_INT_QUICK_ATTACK_PHASE);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_SPECIALUPDUMMY, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialairhiend);
}