use super::*;
#[acmd_script( agent = "snake", script = "game_specialairsaway", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairsaway(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, true, *WEAPON_SNAKE_NIKITA_INSTANCE_WORK_ID_FLAG_AWAY);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }  
}  
pub fn install() {
    smashline::install_acmd_scripts!(
    game_ );
}