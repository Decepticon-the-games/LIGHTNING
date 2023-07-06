use super::*;

#[fighter_frame_callback]
pub fn resets_falses(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        let idles = (status_kind == *FIGHTER_STATUS_KIND_WAIT 
        || status_kind == *FIGHTER_STATUS_KIND_FALL 
        || status_kind == *FIGHTER_STATUS_KIND_FALL_AERIAL);

        let walks_runs_jumps_falls = (status_kind == *FIGHTER_STATUS_KIND_WALK
        || status_kind == *FIGHTER_STATUS_KIND_DASH
        || status_kind == *FIGHTER_STATUS_KIND_TURN_DASH
        || status_kind == *FIGHTER_STATUS_KIND_JUMP
        || status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL);

        if idles || walks_runs_jumps_falls{
            CANCEL_IN_NEUTRAL[entry_id] = false;
        }

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
            AIRDODGE_COUNT[entry_id] = 0;
            VANISH_COUNT[entry_id] = 0;
            CANCEL_IN_NEUTRAL[entry_id] = false;
        }    
    }    
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(resets_falses);
}