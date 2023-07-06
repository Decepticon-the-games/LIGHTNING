use super::*;
#[acmd_script( agent = "falco_illusion", script = "game_moveair", category = ACMD_GAME, low_priority )]
unsafe fn game_moveair(fighter: &mut L2CAgentBase) {
    let oboma = sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID)) as u32); // links weapon to whatever may ownn it  
    let o_entry_id = WorkModule::get_int(&mut *oboma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 80, 60, 0, 68, 4.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 270, 50, 0, 40, 3.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ENABLE_ATTACK_CANCEL[o_entry_id] = true;
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_moveair);
}