
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon};
use smash::lib::lua_const::*;
use smashline::*;
use crate::fighters::common::mechanics::attack_cancels::ENABLE_ATTACK_CANCEL;






#[fighter_frame( agent = FIGHTER_KIND_DEMON )]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(module_accessor);
        let motion_kind = MotionModule::motion_kind(module_accessor);       
        let frame = MotionModule::frame(module_accessor);
        ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        //let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        ////let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
        ////let jump_guard_dash_upspecial_pressed = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0);
    //
        
        //FIXES
        //-------------------------------------------------------------------------------
        
            
        if status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2F {
            if frame <15.0 {
                ENABLE_ATTACK_CANCEL[entry_id] = false;  
            }
            else {
                ENABLE_ATTACK_CANCEL[entry_id] = true; 
            }                
        }
        else if status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2L {
            if frame <27.0 {
                ENABLE_ATTACK_CANCEL[entry_id] = false;  
            }
            else {
                ENABLE_ATTACK_CANCEL[entry_id] = true; 
            } 
        }
        else if status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S {
            if frame <33.0  {
                ENABLE_ATTACK_CANCEL[entry_id] = false;  
            }
            else {
                ENABLE_ATTACK_CANCEL[entry_id] = true; 
            } 
        }
         
        else if (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK) 
        || (status_kind == *FIGHTER_STATUS_KIND_FINAL)
        || (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        || (motion_kind== smash::hash40("attack_14"))
        || (motion_kind== smash::hash40("attack_15"))
        || (motion_kind== smash::hash40("attack_16"))
        || (motion_kind== smash::hash40("attack_17"))
        || (motion_kind== smash::hash40("attack_18"))
        || (motion_kind== smash::hash40("attack_19"))
        || (motion_kind== smash::hash40("attack_100"))
        || (motion_kind== smash::hash40("attack_100_sub"))
        || (motion_kind== smash::hash40("attack_100_end"))
        || (motion_kind== smash::hash40("attack_110"))


        || (motion_kind== smash::hash40("attack_s3_hi"))
        || (motion_kind== smash::hash40("attack_s3_lw"))
        || (motion_kind== smash::hash40("attack_hi_3"))
        //|| (motion_kind== smash::hash40("attack_hi3_2"))

        

        //|| (motion_kind == smash::hash40("attack_stand1") && frame <13.0 )
        || (motion_kind == smash::hash40("attack_stand_21"))
        || (motion_kind == smash::hash40("attack_stand_22"))
        || (motion_kind == smash::hash40("attack_stand_23"))
        || (motion_kind == smash::hash40("attack_stand_24"))
        || (motion_kind == smash::hash40("attack_stand_31"))
        || (motion_kind == smash::hash40("attack_stand_32"))
        || (motion_kind == smash::hash40("attack_stand_4"))
        //|| (motion_kind == smash::hash40("attack_stand5") && frame <10.0 )
        //|| (motion_kind == smash::hash40("attack_stand6") && frame <13.0 )
        //|| (motion_kind == smash::hash40("attack_squat1") && frame <14.0 )
        //|| (motion_kind == smash::hash40("attack_squat2") && frame <6.0 )
        //|| (motion_kind == smash::hash40("attack_squat3") && frame <10.0 )
        //|| (motion_kind == smash::hash40("attack_squat4") && frame <12.0 )
        //|| (motion_kind == smash::hash40("attack_step2") && frame <12.0 )
        {
            ENABLE_ATTACK_CANCEL[entry_id] = false;  
        }
        else {
            ENABLE_ATTACK_CANCEL[entry_id] = true; 
        }

        //ENHANCES
        //--------------------------------------------------------------------------------


        //MOTION CANCELS
        //--------------------------------------------------------------------------------

    }                                   
}

pub fn install() {
    smashline::install_agent_frames!(once_per_fighter_frame);
}