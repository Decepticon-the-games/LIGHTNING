use super::*;
#[acmd_script( agent = "rockman", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn game_attacks3(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_ROCKBUSTER, false, -1);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_SHOOT);
        whiff_cancel(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_attacks3);
}