use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::lib::lua_const::*;
use acmd;

static mut UP_SPECIAL : [bool; 8] = [false; 8];
static mut ENTRY_ID : usize = 0;
pub fn set_bool(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            UP_SPECIAL[ENTRY_ID] = true;
        }
    }
}

pub fn install() {
    acmd::add_custom_hooks!(set_bool);
}