use super::*;
#[acmd_script( agent = "ken", script = "game_speciallwstepf", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallwstepf(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor,  *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 10.0, 0.0, 10.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_speciallwstepf);
}