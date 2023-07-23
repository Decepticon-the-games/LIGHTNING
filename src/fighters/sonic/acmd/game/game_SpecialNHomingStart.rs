use super::*;
#[acmd_script( agent = "sonic", script = "game_specialnhomingstart", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnhomingstart(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 67.0, 0.0, 10.0, 10.0, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_);
}