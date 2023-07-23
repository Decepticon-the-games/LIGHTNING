use super::*;
#[acmd_script( agent = "snake", script = "game_speciallwcheckshootc4", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwcheckshootc4(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    battle_object();
    methodlib::L2CValue::L2CValue(void*)();
    methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
    methodlib::L2CValue::as_pointer()const(*FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *ARTICLE_OPE_TARGET_LAST);
    is_constraint_article();
    if(false){
        if macros::is_excute(fighter) {
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *ARTICLE_OPE_TARGET_ALL, false);
        }
    }  
}  
pub fn install() {
    smashline::install_acmd_scripts!(
    game_ );
}