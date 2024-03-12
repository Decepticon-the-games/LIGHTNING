use super::*;

#[skyline::hook(replace=CancelModule::enable_cancel)]
pub unsafe fn whiff_cancel(fighter: &mut BattleObjectModuleAccessor) -> u64 {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH)
}