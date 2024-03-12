use super::*;
#[acmd_script( agent = "luigi", script = "game_aircatchlanding", category = ACMD_GAME, low_priority )]
unsafe fn game_aircatchlanding(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::IS_EXIST_ARTICLE(fighter, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("air_catch_landing"), false, -1.0);
        }
    }
    if macros::IS_EXIST_ARTICLE(fighter, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER) {
        if macros::is_excute(fighter) {
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_AIR_LASSO_FLAG_SHOOT_PLUNGER);
    //if(methodlib::L2CValue::operator==(lib::L2CValueconst&)const(false, false)){
        if macros::is_excute(fighter) {
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
            whiff_cancel(fighter);
        }
    //}
}    
pub fn install() {
    smashline::install_acmd_scripts!(
     game_aircatchlanding);
}