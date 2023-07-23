use super::*;

#[fighter_frame_callback]
pub fn resets_falses(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        let idles = (status_kind == *FIGHTER_STATUS_KIND_WAIT 
        || status_kind == *FIGHTER_STATUS_KIND_FALL 
        || status_kind == *FIGHTER_STATUS_KIND_FALL_AERIAL);

        /*let walks_runs_jumps_falls = (status_kind == *FIGHTER_STATUS_KIND_WALK
        || status_kind == *FIGHTER_STATUS_KIND_DASH
        || status_kind == *FIGHTER_STATUS_KIND_TURN_DASH
        || status_kind == *FIGHTER_STATUS_KIND_JUMP
        || status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL);*/

        //CANCEL_IN_NEUTRAL will actually run as long as it's true. This line ensures it runs false when enable_cacel is run IN ANY CASE, so that it doesn't overlap on the startup of another move.

        if (AttackModule::is_attack_occur(fighter.module_accessor) && ! status_kind == *FIGHTER_STATUS_KIND_THROW)//Only on whiff, not on hit (DON'T COUNT THROWS CUZ U CANT EVEN WHIFF THOSE, plus some throws have hitboxes)
        || CancelModule::is_enable_cancel(fighter.module_accessor) 
        {//The moment these run, turn off CANCEL_IN_NEUTRAL.
            CANCEL_IN_NEUTRAL[entry_id] = false;
        }

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
            AIRDODGE_COUNT[entry_id] = 0;
            VANISH_COUNT[entry_id] = 0;
            CANCEL_IN_NEUTRAL[entry_id] = false;
        }

        if entry_id <1 {    
            println!("is_enable_cancel: {}", CancelModule::is_enable_cancel(fighter.module_accessor) );
            println!("cancein_neutral: {}", CANCEL_IN_NEUTRAL[entry_id]);
        }
    }    
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(resets_falses);
}