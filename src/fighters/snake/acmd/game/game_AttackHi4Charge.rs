use super::*;
#[acmd_script( agent = "snake", script = "game_attackhi4charge", category = ACMD_GAME, low_priority )]
unsafe fn game_attackhi4charge(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_TRENCHMORTAR, Hash40::new("hold"), false, -1.0);
    }  
}  
pub fn install() {
    smashline::install_acmd_scripts!(
    game_ );
}