use super::*;
#[acmd_script( agent = "murabito_slingshot", script = "game_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn game_attackairb(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *WEAPON_MURABITO_SLINGSHOT_GENERATE_ARTICLE_BULLET, false, -1);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *WEAPON_MURABITO_SLINGSHOT_GENERATE_ARTICLE_BULLET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
        if entry_id < 8 {
            whiff_cancel(fighter); 
        }
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_attackairb);
}