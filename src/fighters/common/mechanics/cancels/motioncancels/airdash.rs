use super::*;

#[fighter_frame_callback]
pub fn airdash(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
          

        if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {

            //Cancel
            if frame >= 8.0 {
                CancelModule::enable_cancel(fighter.module_accessor);
                AIRDASH[entry_id] = true;
            }          
        }
    }
}
/*
#[fighter_frame_callback]
pub fn airstep(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
          

        if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {

            //Cancel
            if frame >= 2.0 && <= 3.0 {
                CancelModule::enable_cancel(fighter.module_accessor);
                AIRDASH[entry_id] = true;
            }          
        }
    }
}
*/
pub fn install() {
    smashline::install_agent_frame_callbacks!(airdash);

} 