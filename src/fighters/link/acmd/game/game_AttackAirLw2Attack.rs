use super::*;
#[acmd_script( agent = "link", script = "game_attackairlw2attack", category = ACMD_GAME, low_priority )]
unsafe fn game_attackairlw2attack(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 65, 80, 0, 50, 4.3, 1.0, 1.0, 0.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        enable_attack_cancel(fighter); 
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_attackairlw2attack);
 }