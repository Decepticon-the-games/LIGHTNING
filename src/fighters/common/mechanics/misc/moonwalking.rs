use super::*;

#[fighter_frame_callback]
pub fn moonwalk_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);   

        //MOONWALKING
        let speed = smash::phx::Vector3f {x: -1.0, y: 0.0, z: 0.0 };
        let mut stick_x = ControlModule::get_stick_x(fighter.module_accessor);
        stick_x = stick_x * PostureModule::lr(fighter.module_accessor);

        if (motion_kind == smash::hash40("dash") || motion_kind == smash::hash40("turn_dash")) && MotionModule::frame(fighter.module_accessor) <= 3.0 && stick_x <= -0.3 {
            KineticModule::add_speed(fighter.module_accessor,  &speed);
        }
    }
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(moonwalk_opff);

} 