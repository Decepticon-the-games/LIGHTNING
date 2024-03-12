use super::*;
use crate::fighters::ness::opff::NO_PKFIRE;
#[acmd_script( agent = "ness", script = "game_specials", category = ACMD_GAME, low_priority )]
unsafe fn game_specials(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.85);
    frame(fighter.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        if NO_PKFIRE[entry_id] {
            ArticleModule::remove_exist(fighter.module_accessor,*FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            NO_PKFIRE[entry_id] = false;
            whiff_cancel(fighter);
        }
        else {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE, false, -1);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_NESS_STATUS_SPECIAL_S_FLAG_SHOOT);
            whiff_cancel(fighter);
        }
    }
    macros::FT_MOTION_RATE(fighter, 1.0);  
}    
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specials);
}

//PK Fire exists once at a time