use super::*;
#[acmd_script( agent = "pfushigisou", script = "game_specialnend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_SEED, false, -1);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }    
}
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialnend );
}