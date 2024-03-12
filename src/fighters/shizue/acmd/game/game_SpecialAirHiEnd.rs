use super::*;
#[acmd_script( agent = "shizue", script = "game_specialairhiend", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairhiend(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_SWING, Hash40::new("air_end"), false, -1.0);
        whiff_cancel(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairhiend);
}