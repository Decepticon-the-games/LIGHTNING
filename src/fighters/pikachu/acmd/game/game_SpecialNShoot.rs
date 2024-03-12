use super::*;
#[acmd_script( agent = "pikachu", script = "game_specialnshoot", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnshoot(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_DENGEKIDAMA, false, -1);
        whiff_cancel(fighter);
    }
} 
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialnshoot);
}