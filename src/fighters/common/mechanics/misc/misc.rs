use super::*;

pub static mut TRANSITION_TERM_NONE : [bool; 8] = [false; 8];
#[fighter_frame_callback]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 1);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
        let frame = MotionModule::frame(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);




//STEP VANISH
        /*if status_kind == *FIGHTER_STATUS_KIND_ESCAPE
        //|| status_kind == *FIGHTER_STATUS_KIND_ESCAPE_F
        //|| status_kind == *FIGHTER_STATUS_KIND_ESCAPE_B 
        {
            
            let hitstatus = HitModule::get_whole(fighter.module_accessor, 0);
            let zero = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};
            let rotation = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};
            //Speedline effect
            EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_attack_speedline"), smash::phx::Hash40::new("body"), &zero, &rotation, 1.0, &zero, &zero, false, 0, 0, 0);
            //Invisible effect
            if hitstatus == *HIT_STATUS_XLU {
                VisibilityModule::set_whole(fighter.module_accessor, false);
                
            }
        }  
        else {
            VisibilityModule::set_whole(fighter.module_accessor, true);
            //macros::EFFECT_OFF_KIND(fighter, smash::phx::Hash40::new("sys_attack_speedline"), true, true);
        }*/         

        
//SHORTHOP SPECIAL
        /*if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {

            }
        }*/





    


// ROLL??
        //if motion_kind== smash::hash40("escape_f") || motion_kind== smash::hash40("escape_b")  {

        //}

//SHIMMY??
        //if status_kind == *FIGHTER_STATUS_KIND_DASH {
        //    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 {
                //PostureModule::reverse_lr(fighter.module_accessor);
        //    }   
        //}

//GROUNDED DODGES ARE FASTER?

//Get airdodge back during free fall
        //if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            //if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD != 0 {
        //    StatusModule::change_status_request_from_script(fighter.module_accessor,*FIGHTER_STATUS_KIND_FALL,true); 
            //}   
        //}

//GRAB COMBOS
        if status_kind == *FIGHTER_STATUS_KIND_THROW {
            /*if AttackModule::is_attack_occur(fighter.module_accessor) {
            
                if ! (fighter_kind == *FIGHTER_KIND_SHULK && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0) { //so shulk's monado arts don't break
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
            }*/
            
               if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
                    //AttackModule::set_power_mul(fighter.module_accessor, 1.0);
                    AttackModule::set_damage_reaction_mul(fighter.module_accessor, 0.4);
                } 
            
            
        }
        else {
            AttackModule::set_reaction_mul(fighter.module_accessor, 1.0);
        }
//
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(once_per_fighter_frame);

} 