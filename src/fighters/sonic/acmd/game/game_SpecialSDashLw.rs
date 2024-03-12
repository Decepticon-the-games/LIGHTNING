use super::*;
#[acmd_script( agent = "sonic", script = "game_specialsdashlw", category = ACMD_GAME, low_priority )]
unsafe fn game_specialsdashlw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
        macros::ATTACK(fighter, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_ATTACK_ID_DEFAULT, 0, Hash40::new("hip"), 4.0, 70, 70, 0, 65, 2.0, 0.0, 1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_captured_same_time_attack(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_ATTACK_ID_DEFAULT, true);
        AttackModule::set_attack_keep_rumble(fighter.module_accessor, 0, true);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_);
}