use super::*;
#[acmd_script( agent = "lucario", script = "game_specialsqigong", category = ACMD_GAME, low_priority )]
unsafe fn game_specialsqigong(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    
    if macros::IS_GENERATABLE_ARTICLE(fighter, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG) {
        if macros::is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG, false, -1);
            CANCEL_IN_NEUTRAL[entry_id] = true;
        }
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialsqigong);
}