use super::*;
#[acmd_script( agent = "trail", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn game_attacks3(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_S3_FLAG_CHECK_COMBO_BUTTON_ON);
    }
    frame(fighter.lua_state_agent, 13.0);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 7.2, 361, 110, 0, 48, 3.4, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.2, 361, 110, 0, 48, 3.4, 0.0, 4.6, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 7.2, 361, 110, 0, 48, 3.4, 0.0, 9.2, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 7.2, 361, 110, 0, 48, 3.4, -3.2, 9.2, -3.6, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
            macros::ATK_SET_SHIELD_SETOFF_MUL2(fighter, 0, 2.4);
            macros::ATK_SET_SHIELD_SETOFF_MUL2(fighter, 1, 2.4);
            macros::ATK_SET_SHIELD_SETOFF_MUL2(fighter, 2, 2.4);
            macros::ATK_SET_SHIELD_SETOFF_MUL2(fighter, 3, 2.4);
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 7.2, 361, 110, 0, 48, 3.4, -3.2, 9.2, -7.6, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    else{
    macros::FT_MOTION_RATE(fighter, 0.8);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 5.2, 361, 14, 0, 24, 3.4, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 5.2, 361, 14, 0, 22, 3.4, 0.0, 4.6, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 5.2, 180, 18, 0, 30, 3.4, 0.0, 9.2, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 5.2, 180, 18, 0, 30, 3.4, -3.2, 9.2, -3.6, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 13.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 13.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 12.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL2(fighter, 0, 3);
        macros::ATK_SET_SHIELD_SETOFF_MUL2(fighter, 1, 3);
        macros::ATK_SET_SHIELD_SETOFF_MUL2(fighter, 2, 3);
        macros::ATK_SET_SHIELD_SETOFF_MUL2(fighter, 3, 3);
    }
}    
wait(fighter.lua_state_agent, 1.0);
if macros::is_excute(fighter) {
    macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 5.2, 180, 18, 0, 30, 3.4, -3.2, 9.2, -7.6, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
    AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 12.0, false);
    macros::ATK_SET_SHIELD_SETOFF_MUL2(fighter, 3, 3);
}
wait(fighter.lua_state_agent, 2.0);
if macros::is_excute(fighter) {
    macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 5.2, 361, 14, 0, 24, 3.4, 0.0, 9.2, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
    AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 12.0, false);
    macros::ATK_SET_SHIELD_SETOFF_MUL2(fighter, 2, 3);
}
wait(fighter.lua_state_agent, 3.0);
if macros::is_excute(fighter) {
    AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
}
macros::FT_MOTION_RATE(fighter, 1.0);
frame(fighter.lua_state_agent, 29.0);
if macros::is_excute(fighter) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
}
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_attacks3);
}
 