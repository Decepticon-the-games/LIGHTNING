use super::*;

#[fighter_frame_callback]
pub fn parries_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        
        //REWARD PARRIES WITH INVINCIBILITY

        if motion_kind == smash::hash40("just_shield") || motion_kind == smash::hash40("just_shield_off") {
            HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);    
            
        }
    }   
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(parries_opff);
} 