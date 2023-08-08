use super::*;

#[fighter_frame_callback]
pub fn airdodge(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);  


        if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
           
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {

                //Airdash
                if frame >= 8.0 {
                    CancelModule::enable_cancel(fighter.module_accessor);
                }      
            }
        }
    }
}

    

pub fn install() {
    smashline::install_agent_frame_callbacks!(airdodge);
} 