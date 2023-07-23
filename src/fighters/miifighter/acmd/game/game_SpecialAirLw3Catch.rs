use super::*;
#[acmd_script( agent = "miifighter", script = "game_specialairlw3catch", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlw3catch(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        COUNTER_CANCEL[entry_id] = true;
        macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 15.0, 0.0, 5.0, 10.5, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FI, *COLLISION_PART_MASK_ALL, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairlw3catch);
}