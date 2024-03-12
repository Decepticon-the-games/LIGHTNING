use super::*;
//wavestep
#[fighter_frame_callback]
pub fn wavestep(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

        let end_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("jump_squat_frame"), 0);
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_JUMP_SQUAT 
        && MotionModule::frame(fighter.module_accessor) >= end_frame as f32 { 
            //&& MotionModule::is_end(fighter.module_accessor) {
        
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_ESCAPE);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
        }

        
        
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_LANDING {
            if MotionModule::frame(fighter.module_accessor) >=1.0
            && MotionModule::frame(fighter.module_accessor) <=3.0
            {
                
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
                    WAVESTEP[entry_id] = true;
                }
                else {
                    WAVESTEP[entry_id] = false;
                }
            }
        }
        
        if WAVESTEP[entry_id] {
            //less grounded friction, to transfer slide
            //CancelModule::enable_cancel(fighter.module_accessor);
            WAVESTEP[entry_id] = false;
        }        
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(wavestep);
} 