use super::*;
#[acmd_script( agent = "demon", script = "game_throwb", category = ACMD_GAME, low_priority )]
unsafe fn game_throwb(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if !smash_rs::app::FighterCutInManager::is_vr_mode(){
        if smash_rs::app::FighterCutInManager::is_one_on_one_including_thrown(&*(fighter.module_accessor as *const smash_rs::app::BattleObjectModuleAccessor)) {
            if macros::is_excute(fighter) {
                FighterSpecializer_Demon::check_disabled_motion_camera_of_scale(fighter.module_accessor);
                FighterSpecializer_Demon::check_disabled_motion_camera_of_stage(fighter.module_accessor);
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_THROW_MOTION_CAMERA) {
                if macros::is_excute(fighter) {
                    macros::CHECK_VALID_START_CAMERA(fighter, 0, 7, 0, 35, 0, 0, false);
                }
            }
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_THROW_MOTION_CAMERA) {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
                    if macros::is_excute(fighter) {
                        macros::REQ_MOTION_CAMERA(fighter, Hash40::new("e01throwb.nuanmb"), false);
                    }
                }
            }
            if macros::is_excute(fighter) {
                if macros::is_excute(fighter) {
                    let scale = PostureModule::scale(fighter.module_accessor);
                    macros::CAM_ZOOM_IN_arg5(fighter, 7.0, 0.0, scale * 1.5, 0.0, 0.0);
                }
            }
        }
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
    macros::REVERSE_LR(fighter);
    macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 58, 340, 0, 34, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
    macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 58, 55, 0, 60, 4.0, 0.0, 2.0, -12.0, Some(0.0), Some(2.0), Some(-22.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_BODY);
    AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    macros::CHECK_FINISH_CAMERA(fighter, 17, 0);
    }
    frame(fighter.lua_state_agent, 47.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
CANCEL_IN_NEUTRAL[entry_id] = true;
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
            let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
            let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        macros::CAM_ZOOM_OUT(fighter);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_throwb);
}