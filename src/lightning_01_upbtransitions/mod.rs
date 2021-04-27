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
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI && ! (fighter_kind == *FIGHTER_KIND_CAPTAIN || fighter_kind == FIGHTER_KIND_GANON) {
            UP_SPECIAL[ENTRY_ID] = true;
        }
        //Reset up b once you touch the ground
        if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
            UP_SPECIAL[ENTRY_ID] = false;
        }
        // No Free Fall? Get jump back after up b
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
               StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_JUMP_AERIAL,true); 
            }
            
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