use super::*;
#[acmd_script( agent = "nana", script = "game_specialsattackbothsub", category = ACMD_GAME, low_priority )]
unsafe fn game_specialsattackbothsub(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 25, 60, 0, 25, 3.5, 0.0, 7.5, 4.5, None, None, None, 0.7, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.2, 32, 60, 0, 13, 4.5, 0.0, 8.0, 10.0, None, None, None, 0.7, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.2, 25, 50, 0, 25, 3.5, 0.0, 7.5, -4.0, None, None, None, 0.7, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.2, 32, 50, 0, 13, 4.5, 0.0, 8.0, -10.0, None, None, None, 0.7, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        ENABLE_ATTACK_CANCEL[entry_id] = true; 
    } 
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialsattackbothsub);
}