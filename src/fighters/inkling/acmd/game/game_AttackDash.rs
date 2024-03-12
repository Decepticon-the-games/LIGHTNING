use super::*;
#[acmd_script( agent = "inkling", script = "game_attackdash", category = ACMD_GAME, low_priority )]
unsafe fn game_attackdash(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID) {
        if macros::is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, -1);
        }
    }
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    let rate = MotionModule::rate(fighter.module_accessor);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new_raw(motion), false, -1.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_frame(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, MotionModule::frame(fighter.module_accessor));
        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, rate);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID);
        let status = StatusModule::status_kind(fighter.module_accessor);
        if status != *FIGHTER_STATUS_KIND_REBIRTH {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
        }
    }
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("attack_dash_l"), false, -1.0);
        }
    }
    else{
    if macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("attack_dash_r"), false, -1.0);
    }
}
 
frame(fighter.lua_state_agent, 1.0);
if macros::is_excute(fighter) {
    VisibilityModule::set_whole(fighter.module_accessor, true);
    ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
}
frame(fighter.lua_state_agent, 6.0);
if macros::is_excute(fighter) {
    ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
}
frame(fighter.lua_state_agent, 8.0);
if macros::is_excute(fighter) {
    enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 60, 43, 0, 110, 4.0, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ELBOW);
    macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.5);
}
wait(fighter.lua_state_agent, 2.0);
if macros::is_excute(fighter) {
    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 60, 43, 0, 100, 3.0, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ELBOW);
    macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.5);
}
wait(fighter.lua_state_agent, 4.0);
if macros::is_excute(fighter) {
    AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
}
}    
pub fn install() {
    smashline::install_acmd_scripts!(
        game_attackdash);
}