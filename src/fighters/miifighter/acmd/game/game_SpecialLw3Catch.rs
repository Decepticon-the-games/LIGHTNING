use super::*;
#[acmd_script( agent = "miifighter", script = "game_speciallw3catch", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallw3catch(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        COUNTER_CANCEL[entry_id] = true;
        macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 8.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(7.5), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_ALL, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
    game_speciallw3catch);
}