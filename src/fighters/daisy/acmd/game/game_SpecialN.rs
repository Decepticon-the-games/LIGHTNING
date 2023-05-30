use super::*;
#[acmd_script( agent = "daisy", script = "game_specialn", category = ACMD_GAME, low_priority )]
unsafe fn game_specialn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, *FIGHTER_PEACH_SHIELD_KIND_KINOPIO_GUARD, *FIGHTER_PEACH_SHIELD_GROUP_KIND_KINOPIO_GUARD);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, *FIGHTER_PEACH_SHIELD_KIND_KINOPIO_GUARD, *FIGHTER_PEACH_SHIELD_GROUP_KIND_KINOPIO_GUARD);
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KINOPIO, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialn);
}