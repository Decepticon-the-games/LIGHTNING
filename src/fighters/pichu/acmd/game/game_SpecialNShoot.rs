use super::*;
#[acmd_script( agent = "pichu", script = "game_specialnshoot", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnshoot(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICHU_GENERATE_ARTICLE_DENGEKIDAMA, false, -1);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialnshoot);
}