use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::FighterManager;
use smash::app::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::phx::Vector2f;
use acmd::*;
use crate::lightning_01_ultrainstinct::SECRET_SENSATION;
use crate::lightning_01_upbtransitions::UP_SPECIAL;

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term )]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let ret = original!()(module_accessor,term);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    
    if SECRET_SENSATION[entry_id] {
        return false;
    }
    if UP_SPECIAL[entry_id] {
        if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI {
            return false;
        }
        else {
            return ret;
        }
    }
    else {
        return ret;
    }

}
pub fn install() {
    skyline::install_hook!(is_enable_transition_term_replace);
}