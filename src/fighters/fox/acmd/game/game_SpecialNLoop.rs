use super::*;
#[acmd_script( agent = "fox", script = "game_specialnloop", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnloop(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        CANCEL_IN_NEUTRAL[entry_id] = true;
        }

    if macros::IS_EXIST_ARTICLE(fighter, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER) {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, Hash40::new("loop"), false, -1.0);
        }
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FOX_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialnloop);
}