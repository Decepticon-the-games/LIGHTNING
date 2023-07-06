use super::*;
#[acmd_script( agent = "mewtwo", script = "game_specialnstart", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnstart(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MEWTWO_GENERATE_ARTICLE_SHADOWBALL, false, -1);
    }
}
pub fn install() {
smashline::install_acmd_scripts!(
   game_ );
}