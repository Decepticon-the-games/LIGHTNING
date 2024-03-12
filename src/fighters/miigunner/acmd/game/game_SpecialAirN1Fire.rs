use super::*;
#[acmd_script( agent = "miigunner", script = "game_specialairn1fire", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairn1fire(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_GUNNERCHARGE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_FLAG_SHOOT);
        whiff_cancel(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairn1fire);
}