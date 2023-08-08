use super::*;
use crate::fighters::common::mechanics::lightning_mechanics::lightning_mode::LIGHTNING;

#[fighter_frame_callback]
pub fn airdash_effect_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        
        if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {

            //Speedline effect
            if fighter.global_table[0xE].get_f32() == 1.0 {

                let dir_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
                let dir_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
                let stick_degrees = dir_y.atan2(dir_x).to_degrees();
                let zero = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};
                let rotation = smash::phx::Vector3f {x:0.0,y:0.0,z:stick_degrees};
                if LIGHTNING[entry_id] {
                    EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_attack_speedline"), smash::phx::Hash40::new("waist"), &zero, &rotation, 1.0, &zero, &zero, false, 0, 0, 0);              
                    macros::BURN_COLOR(fighter, 0.0, 0.784, 1.0, 0.7);
                }
                else {
                    EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_attack_speedline"), smash::phx::Hash40::new("waist"), &zero, &rotation, 1.0, &zero, &zero, false, 0, 0, 0);              
                }
            }
        }
        else {
            macros::EFFECT_OFF_KIND(fighter, smash::phx::Hash40::new("sys_attack_speedline"), true, true);
        }
    }
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(airdash_effect_opff);

}