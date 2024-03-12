use super::*;
#[acmd_script( agent = "brave", script = "game_speciallw9", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallw9(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(fighter, 4.0);
    frame(fighter.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_STRENGTH);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_CRASH, false, -1);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_GENERATED_ARTICLE);
        SlowModule::set_whole(fighter.module_accessor, 20, 30);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_SLOW_WHOLE);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::FT_SET_FINAL_FEAR_FACE(fighter, 30);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_DEATH_RESERVE);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_speciallw9);
}