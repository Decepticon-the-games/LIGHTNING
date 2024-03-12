use super::*;
#[acmd_script( agent = "edge", script = "game_specialairlwhit", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlwhit(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, 0, *FIGHTER_EDGE_SHIELD_GROUP_KIND_SPECIAL_LW);
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_LW_FLAG_IS_HIT_SHIELD) {
        if macros::is_excute(fighter) {
            macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
            COUNTER_CANCEL[entry_id] = true; 
        }
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_EDGE_GENERATE_ARTICLE_FLASH, false, -1);
            whiff_cancel(fighter);
        }
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
        }
        frame(fighter.lua_state_agent, 26.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_LW_FLAG_ENABLE_CONTROL_X);
        }
        frame(fighter.lua_state_agent, 28.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_LW_FLAG_ENABLE_FALL_SPEED);
        }
    }
    else {
        frame(fighter.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(fighter, 2.0);
        frame(fighter.lua_state_agent, 5.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_EDGE_GENERATE_ARTICLE_FLASH, false, -1);
            whiff_cancel(fighter);
        }
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_LW_FLAG_ENABLE_CONTROL_X);
        }
        frame(fighter.lua_state_agent, 20.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_LW_FLAG_ENABLE_FALL_SPEED);
        }
    }

    frame(fighter.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(fighter, 1.15);
    frame(fighter.lua_state_agent, 50.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairlwhit);
}