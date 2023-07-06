use super::*;
use crate::fighters::fox::opff::ILLUSION_CANCEL;
#[acmd_script( agent = "fox_illusion", script = "game_moveair", category = ACMD_GAME, low_priority )]
unsafe fn game_moveair(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 148, 0, 15, 4.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        if AttackModule::is_attack_occur(fighter.module_accessor) {
            ILLUSION_CANCEL[entry_id] = true; 
        } 
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_moveair);
}