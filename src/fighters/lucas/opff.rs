use super::*;
use crate::fighters::common::mechanics::cancels::motioncancels::{CANCEL_IN_NEUTRAL, DISABLE_CANCEL_IN_NEUTRAL, PROJECTILE_COUNT, PROJECTILE_COUNTER};


#[fighter_frame( agent = FIGHTER_KIND_LUCAS )]

    pub fn lucas_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            let status_kind = StatusModule::status_kind(module_accessor);
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            //let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);



//Limit side special cancel to 2 times before having to 30 frames to reset it

            //Counter
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
                if CANCEL_IN_NEUTRAL[entry_id] {
                    if PROJECTILE_COUNTER[entry_id] == false {
                        PROJECTILE_COUNT[entry_id] +=1;
                        PROJECTILE_COUNTER[entry_id] = true;
                    }
                }
                else {
                    PROJECTILE_COUNTER[entry_id] = false;
                } 
            }
            else {
                PROJECTILE_COUNTER[entry_id] = false;
            }

            //Condition
            if PROJECTILE_COUNT[entry_id] >= 2 {  
                if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {// So that  CANCEL_IN_NEUTRAL remains disabled until the status is finished                
                    PROJECTILE_COUNT[entry_id] = 2;
                    DISABLE_whiff_cancel(fighter); 
                } 
                //Reset
                else {
                    DISABLE_CANCEL_IN_NEUTRAL[entry_id] = false; 
                    PROJECTILE_COUNT[entry_id] = 0;  
                }
            }




            //RESETS
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
                PROJECTILE_COUNT[entry_id] = 0;
                DISABLE_CANCEL_IN_NEUTRAL[entry_id] = false;
            } 

        }
    }

pub fn install() {
    smashline::install_agent_frames!(lucas_opff);

}