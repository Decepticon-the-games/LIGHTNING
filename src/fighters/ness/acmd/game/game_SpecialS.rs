use super::*;
use crate::fighters::ness::opff::NO_PKFIRE;
#[acmd_script( agent = "ness", script = "game_specials", category = ACMD_GAME, low_priority )]
unsafe fn game_specials(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if NO_PKFIRE[entry_id] {
        frame(fighter.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(fighter, 0.85);
        frame(fighter.lua_state_agent, 20.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        frame(fighter.lua_state_agent, 21.0);
        if macros::is_excute(fighter) {
            CANCEL_IN_NEUTRAL[entry_id] = true; 
            NO_PKFIRE [entry_id] = false; 
        }
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    else {
        frame(fighter.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(fighter, 0.85);
        frame(fighter.lua_state_agent, 20.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        frame(fighter.lua_state_agent, 21.0);
        if macros::is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE, false, -1);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_S_FLAG_SHOOT);
            CANCEL_IN_NEUTRAL[entry_id] = true; 
        }
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specials);
}

//PK Fire exists once at a time