use super::*;
#[acmd_script( agent = "snake", script = "game_specialnthrowlw", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnthrowlw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 9.0);
    /*battle_object();
    methodlib::L2CValue::L2CValue(void*)();
    methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
    methodlib::L2CValue::as_pointer()const(*FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, *ARTICLE_OPE_TARGET_LAST);
    is_constraint_article();
    if(false){*/
        if macros::is_excute(fighter) {
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
            whiff_cancel(fighter);
        }
    //}  
}  
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialnthrowlw );
}