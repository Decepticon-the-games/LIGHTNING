use super::*;
#[acmd_script( agent = "demon", script = "game_throwhi", category = ACMD_GAME, low_priority )]
unsafe fn game_throwhi(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 2.0, 76, 45, 10, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(fighter.lua_state_agent, 38.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, false, -1);
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_DEMON_GENERATE_ARTICLE_BLASTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        CANCEL_IN_NEUTRAL[entry_id] = true;
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 43.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 48.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 2.0);
    frame(fighter.lua_state_agent, 56.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_SPECIAL_N_FLAG_FOLLOW_NODE);
    }
    frame(fighter.lua_state_agent, 63.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
    frame(fighter.lua_state_agent, 70.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 72.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 73.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 74.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 8.0);
    frame(fighter.lua_state_agent, 75.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, false, 0.0);
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_throwhi);
}