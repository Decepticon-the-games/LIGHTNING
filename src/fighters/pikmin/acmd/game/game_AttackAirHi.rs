use super::*;
use crate::fighters::pikmin::opff::PIKMIN_PIKMIN_ATTACK_CANCEL;
#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairhi", category = ACMD_GAME, low_priority )]
unsafe fn game_attackairhi(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

frame(fighter.lua_state_agent, 7.0);
if macros::is_excute(fighter) {
    PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id] = true;
macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 12.6, 95, 73, 0, 50, 3.3, 0.0, 3.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
    macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_XLU);
}
wait(fighter.lua_state_agent, 8.0);
if macros::is_excute(fighter) {
    AttackModule::clear_all(fighter.module_accessor);
    whiff_cancel(fighter);
    HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
}
} 

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairhi_b", category = ACMD_GAME, low_priority )]
unsafe fn game_attackairhi_b(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id] = true;
macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 9.0, 95, 92, 0, 50, 3.3, 0.0, 3.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
        macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_XLU);
    }
    wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairhi_v", category = ACMD_GAME, low_priority )]
unsafe fn game_attackairhi_v(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id] = true;
macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 14.4, 95, 73, 0, 50, 3.3, 0.0, 3.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
        macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_XLU);
    }
    wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairhi_w", category = ACMD_GAME, low_priority )]
unsafe fn game_attackairhi_w(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id] = true;
macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 7.2, 95, 92, 0, 50, 3.3, 0.0, 3.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
        macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_XLU);
    }
    wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairhi_y", category = ACMD_GAME, low_priority )]
unsafe fn game_attackairhi_y(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id] = true;
macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 9.0, 95, 92, 0, 50, 4.95, 0.0, 3.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PIKMIN);
        macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_XLU);
    }
    wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}


pub fn install() {
    smashline::install_acmd_scripts!(
    game_attackairhi,
    game_attackairhi_b,
    game_attackairhi_v,
    game_attackairhi_w,
    game_attackairhi_y
);
}
