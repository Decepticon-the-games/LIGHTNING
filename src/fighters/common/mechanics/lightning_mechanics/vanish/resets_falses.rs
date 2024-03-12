use super::*;

//RSETS
#[fighter_frame_callback]
pub fn resets_falses(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        //RESETS      
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
            FIGHTER_STATUS_KIND_VANISH[entry_id] = false;
            DEFENDER_VANISH[entry_id] = false;
            DIRECT_HIT[entry_id] = false;  
            PROJECTILE_HIT[entry_id] = false;  
            FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_VANISH[entry_id] = false; 
        } 
        if smash::app::sv_information::is_ready_go() == false {
            VANISH_COUNT[entry_id] = 0;
        }
    }
}

pub fn install() {

    smashline::install_agent_frame_callbacks!(resets_falses);
}