use super::*;

#[fighter_frame_callback]
pub fn resets_falses(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

        if CANCEL_IN_NEUTRAL[entry_id] {
            if StopModule::is_hit(fighter.module_accessor)
            //|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE
            //|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR
            //|| MotionModule::is_end(fighter.module_accessor)
            //|| (ATTACK_CANCEL[entry_id] && is_after_hitlag(fighter))
            {//The moment these run, turn off CANCEL_IN_NEUTRAL.
                CANCEL_IN_NEUTRAL[entry_id] = false; 
                AIRSTEP[entry_id] = false;
            }            
        }
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
            AIRDODGE_COUNT[entry_id] = 0;
            CANCEL_IN_NEUTRAL[entry_id] = false;  
            AIRSTEP[entry_id] = false;
        }
    }    
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(resets_falses);
}