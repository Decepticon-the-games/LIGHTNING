use super::*;
#[acmd_script( agent = "jack", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhi(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;



    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, Hash40::new("special_hi"), false, -1.0);
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, 0);
        sv_animcmd::SET_RATE_ARTICLE(fighter.lua_state_agent);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_HI_FLAG_REVERSE_LR);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_EXTEND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        AreaModule::reset_area(fighter.module_accessor, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
        macros::ENABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
        AreaModule::reset_area(fighter.module_accessor, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
        macros::ENABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::SEARCH(fighter, 0, 0, Hash40::new("throw"), 3.0, 1.5, 1.5, 0.7, Some(1.5), Some(-15.0), Some(-6.4), *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false);
        macros::SEARCH(fighter, 1, 0, Hash40::new("throw"), 5.5, 1.5, 1.5, 0.7, None, None, None, *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false);
        macros::SEARCH(fighter, 2, 0, Hash40::new("throw"), 5.5, 1.2, 1.5, 0.7, None, None, None, *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 1, false);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::SET_SEARCH_SIZE_EXIST(fighter, 2, 8);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::UNABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_BACK, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        macros::UNABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialhi);
}
 