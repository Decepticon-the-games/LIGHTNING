use super::*;
use crate::fighters::kamui::opff::DH_CANCEL;
#[acmd_script( agent = "kamui", script = "game_specialnend1", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnend1(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_N_FLAG_AIR_CONTROL);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        
    }
}
#[acmd_script( agent = "kamui", script = "game_specialnend2", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnend2(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_N_FLAG_AIR_CONTROL);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "kamui_dragonhand", script = "game_dhspecialnend1", category = ACMD_GAME, low_priority )]
unsafe fn game_dhspecialnend1(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLAG_IS_KAMUI) {
        if macros::is_excute(fighter) {
            DH_CANCEL[entry_id] = true; 
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 50, 100, 0, 50, 5.5, 0.0, 9.0, 15.0, Some(0.0), Some(9.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 18.0, 50, 100, 0, 50, 6.5, 0.0, 9.0, 15.0, Some(0.0), Some(9.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
            WorkModule::get_float(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
            macros::ATK_LERP_RATIO(fighter, 1026626521);
        }
    }
    else if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            DH_CANCEL[entry_id] = true; 
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 50, 100, 0, 50, 5.5, 0.0, 8.8, 15.0, Some(0.0), Some(8.8), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 18.0, 50, 100, 0, 50, 6.5, 0.0, 8.8, 15.0, Some(0.0), Some(8.8), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        }
    }
    else{
        if macros::is_excute(fighter) {
            DH_CANCEL[entry_id] = true; 
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 50, 100, 0, 50, 5.5, 0.0, 8.1, 15.0, Some(0.0), Some(8.1), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 18.0, 50, 100, 0, 50, 6.5, 0.0, 8.1, 15.0, Some(0.0), Some(8.1), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        }
    }
    if macros::is_excute(fighter) {
        attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
        WorkModule::get_float(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
        macros::ATK_LERP_RATIO(fighter, 1026626521);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
}

#[acmd_script( agent = "kamui_dragonhand", script = "game_dhspecialnend2", category = ACMD_GAME, low_priority )]
unsafe fn game_dhspecialnend2(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLAG_IS_KAMUI) {
        if macros::is_excute(fighter) {
            DH_CANCEL[entry_id] = true; 
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 50, 100, 0, 50, 5.5, 0.0, 9.0, 22.0, Some(0.0), Some(9.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 18.0, 50, 100, 0, 50, 6.5, 0.0, 9.0, 22.0, Some(0.0), Some(9.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
            WorkModule::get_float(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
            macros::ATK_LERP_RATIO(fighter, 1026626521);
        }
    }
    else if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            DH_CANCEL[entry_id] = true; 
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 50, 100, 0, 50, 5.5, 0.0, 8.9, 22.0, Some(0.0), Some(8.9), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 18.0, 50, 100, 0, 50, 6.5, 0.0, 8.9, 22.0, Some(0.0), Some(8.9), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        }
    }
    else{
        if macros::is_excute(fighter) {
            DH_CANCEL[entry_id] = true; 
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 50, 100, 0, 50, 5.5, 7.0, 12.0, 22.0, Some(0.0), Some(12.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 18.0, 50, 100, 0, 50, 6.5, 7.0, 12.0, 22.0, Some(0.0), Some(12.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        }
    }
    if macros::is_excute(fighter) {
        attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
        WorkModule::get_float(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLOAT_HOLD_RATE);
        macros::ATK_LERP_RATIO(fighter, 1026626521);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
}

#[acmd_script( agent = "kamui_dragonhand", script = "game_dhspecialnendmax", category = ACMD_GAME, low_priority )]
unsafe fn game_dhspecialnendmax(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_KAMUI_DRAGONHAND_INSTANCE_WORK_ID_FLAG_IS_KAMUI) {
        if macros::is_excute(fighter) {
            DH_CANCEL[entry_id] = true; 
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 50, 100, 0, 28, 7.0, 0.0, 9.0, 22.0, Some(0.0), Some(9.0), Some(14.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        }
    }
    else if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            DH_CANCEL[entry_id] = true; 
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 50, 100, 0, 28, 7.0, 0.0, 8.9, 22.0, Some(0.0), Some(8.9), Some(14.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        }
    }
    else{
        if macros::is_excute(fighter) {
            DH_CANCEL[entry_id] = true; 
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 50, 100, 0, 28, 7.0, 7.0, 12.0, 22.0, Some(0.0), Some(12.0), Some(14.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        }
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
        //game_specialnend1,
        //game_specialnend2,
        //game_dhspecialnend1, 
        //ame_dhspecialnend2, 
        //game_dhspecialnendmax
    );
}