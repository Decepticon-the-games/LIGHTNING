use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use acmd;

static mut UP_SPECIAL : [bool; 8] = [false; 8];
static mut ENTRY_ID : usize = 0;
pub fn set_bool(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        //Set up bool to make up bs to only be used once in the air no matter what
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI 
        || StatusModule::status_kind(module_accessor) == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP {
            UP_SPECIAL[ENTRY_ID] = true;
        }
        //Reset up b once you touch the ground, or if captain falcon/ganondorf up b because they naturally gain it again after grabbing
        if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND 
        || StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_CLIFF
        || StatusModule::status_kind(module_accessor) == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_HI_CLING 
        || StatusModule::status_kind(module_accessor) == *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING {
            UP_SPECIAL[ENTRY_ID] = false;
        }
    
    }
}

#[skyline::hook(replace=smash::app::lua_bind::WorkModule::is_enable_transition_term)]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let ret = original!()(module_accessor,term);
    
    ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if UP_SPECIAL[ENTRY_ID] {
        //Use up b only once in the air 
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
    acmd::add_custom_hooks!(set_bool);
    skyline::install_hook!(is_enable_transition_term_replace);  
}