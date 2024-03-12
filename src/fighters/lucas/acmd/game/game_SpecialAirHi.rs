use super::*;
#[acmd_script( agent = "lucas", script = "game_specialairhi", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairhi(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_LUCAS_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 5.0, 367, 100, 50, 0, 9.0, 3.0, -3.0, 0.0, None, None, None, 0.3, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 5.0, 367, 100, 50, 0, 9.0, -2.0, 0.0, 0.0, None, None, None, 0.3, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 1.0);
    for _ in 0..5 {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 2.0, 367, 50, 50, 30, 3.5, 0.0, 5.0, 6.5, Some(0.0), Some(-1.0), Some(6.5), 0.5, 2.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        macros::ATTACK(fighter, 1, 0, Hash40::new("rot"), 2.0, 366, 50, 0, 50, 4.3, 0.0, 1.0, 1.8, Some(0.0), Some(1.0), Some(-2.2), 0.5, 2.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        macros::ATTACK(fighter, 2, 0, Hash40::new("rot"), 2.0, 366, 50, 0, 50, 6.0, 0.0, 1.0, 1.8, Some(0.0), Some(1.0), Some(-2.2), 0.5, 2.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        AttackModule::set_attack_composition_speed(fighter.module_accessor, 0, true);
        AttackModule::set_attack_composition_speed(fighter.module_accessor, 1, true);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_);
}
if macros::is_excute(fighter) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_ATTACK_FALL_START);
}
for _ in 0..5 {
if macros::is_excute(fighter) {
    macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 1.5, 367, 100, 120, 0, 3.5, 0.0, 5.0, 6.5, Some(0.0), Some(-1.0), Some(6.5), 0.5, 2.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
    macros::ATTACK(fighter, 1, 0, Hash40::new("rot"), 1.5, 363, 100, 130, 0, 4.0, 0.0, 1.0, 1.8, Some(0.0), Some(1.0), Some(-2.2), 0.5, 2.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
    macros::ATTACK(fighter, 2, 0, Hash40::new("rot"), 1.5, 363, 100, 180, 0, 5.7, 0.0, 1.0, 1.8, Some(0.0), Some(1.0), Some(-2.2), 0.5, 2.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
}
wait(fighter.lua_state_agent, 2.0);
if macros::is_excute(fighter) {
    AttackModule::clear_all(fighter.module_accessor);
}
wait(fighter.lua_state_agent, 1.0);
}
if macros::is_excute(fighter) {
enable_attack_cancel(fighter); 
macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 10.0, 50, 74, 0, 90, 12.0, 1.0, -1.0, 0.0, None, None, None, 1.3, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_CRITICAL);
}
wait(fighter.lua_state_agent, 1.0);
if macros::is_excute(fighter) {
WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_NO_LAST_ATTACK);
}
wait(fighter.lua_state_agent, 1.0);
if macros::is_excute(fighter) {
AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_ATTACK_END);
WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_LANDING_ENABLE);
JostleModule::set_status(fighter.module_accessor, true);
GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_LUCAS_CLIFF_HANG_DATA_START as u32);
}
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairhi);
}