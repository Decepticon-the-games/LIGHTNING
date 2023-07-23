use super::*;
use crate::fighters::pikmin::opff::PIKMIN_PIKMIN_ATTACK_CANCEL;
#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairf", category = ACMD_GAME, low_priority )]
unsafe fn game_attackairf(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id] = true;
macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 11.9, 361, 72, 0, 32, 3.0, 1.0, 3.5, 1.0, Some(0.0), Some(-1.6), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
        macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        CANCEL_IN_NEUTRAL[entry_id] = true;
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}      

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairf_b", category = ACMD_GAME, low_priority )]
unsafe fn game_attackairf_b(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id] = true;
macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 8.5, 361, 90, 0, 32, 3.0, 1.0, 3.5, 1.0, Some(0.0), Some(-1.6), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
        macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        CANCEL_IN_NEUTRAL[entry_id] = true;
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairf_v", category = ACMD_GAME, low_priority )]
unsafe fn game_attackairf_v(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id] = true;
macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 13.6, 361, 72, 0, 32, 3.0, 1.0, 3.5, 1.0, Some(0.0), Some(-1.6), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
        macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        CANCEL_IN_NEUTRAL[entry_id] = true;
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairf_w", category = ACMD_GAME, low_priority )]
unsafe fn game_attackairf_w(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id] = true;
macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 6.8, 361, 90, 0, 32, 3.0, 1.0, 3.5, 1.0, Some(0.0), Some(-1.6), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
        macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        CANCEL_IN_NEUTRAL[entry_id] = true;
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairf_y", category = ACMD_GAME, low_priority )]
unsafe fn game_attackairf_y(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id] = true;
macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 8.5, 361, 90, 0, 32, 4.5, 1.0, 3.5, 1.0, Some(0.0), Some(-1.6), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PIKMIN);
        macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        CANCEL_IN_NEUTRAL[entry_id] = true;
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
    game_attackairf,
    game_attackairf_b,
    game_attackairf_v,
    game_attackairf_w,
    game_attackairf_y
);
}

