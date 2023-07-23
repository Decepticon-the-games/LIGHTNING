use super::*;
#[acmd_script( agent = "pzenigame", script = "game_specialnshot", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnshot(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PZENIGAME_STATUS_SPECIAL_N_FLAG_SHOOT_ANGLE_ENABLE);
    }
    macros::FT_MOTION_RATE(fighter, 0.85);
    frame(fighter.lua_state_agent, 7.0);
    for _ in 0..6 {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PZENIGAME_GENERATE_ARTICLE_WATER, false, -1);
    }
    wait(fighter.lua_state_agent, 4.0);
}
frame(fighter.lua_state_agent, 40.0);
if macros::is_excute(fighter) {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PZENIGAME_STATUS_SPECIAL_N_FLAG_SHOOT_ANGLE_ENABLE);
}
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialnshot );
}