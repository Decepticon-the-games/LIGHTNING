use super::*;

//RSETS
#[fighter_frame_callback]
pub fn resets_falses(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        //RESETS      
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
            VANISH[entry_id] = false;
            DEFENDER_VANISH[entry_id] = false;
            CANCEL_INTO_VANISH[entry_id] = false;
            DIRECT_HIT[entry_id] = false;  
            PROJECTILE_HIT[entry_id] = false;  
            ENABLE_CANCEL_INTO_VANISH[entry_id] = false; 
        } 
        if smash::app::sv_information::is_ready_go() == false {
            VANISH_COUNT[entry_id] = 0;
            CANCEL_INTO_VANISH[entry_id] = false;
            
        }
        if ENABLE_CANCEL_INTO_VANISH[entry_id] {
            if CancelModule::is_enable_cancel(fighter.module_accessor)
            || VANISH[entry_id] {
                ENABLE_CANCEL_INTO_VANISH[entry_id] = false;      
            }
        }
    }
}

pub fn install() {

    smashline::install_agent_frame_callbacks!(resets_falses);
}