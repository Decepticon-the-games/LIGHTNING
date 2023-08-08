use super::*;
use crate::fighters::common::mechanics::cancels::attack_cancels::{ENABLE_ATTACK_CANCEL, ENABLE_MULTIHIT_CANCEL, MOVEMENT_CANCEL};


#[fighter_frame( agent = FIGHTER_KIND_YOUNGLINK )]

    pub fn younglink_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            let status_kind = StatusModule::status_kind(module_accessor);
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            //let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);

//Cancel Up smash up to 2 times         

            static mut UPSMASH_CANCEL_COUNT : [bool; 8] = [false; 8];
            static mut UPSMASH_CANCEL_COUNTER : [i32; 8] = [0; 8];

            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
                if AttackModule::is_attack_occur(fighter.module_accessor) {
                    //Counter 
                    if UPSMASH_CANCEL_COUNT[entry_id] == false {
                        UPSMASH_CANCEL_COUNTER[entry_id] +=1;
                        UPSMASH_CANCEL_COUNT[entry_id] = true; 
                    }
                }
                else {
                    UPSMASH_CANCEL_COUNT[entry_id] = false;
                }
                //Disable cancel
                if UPSMASH_CANCEL_COUNTER[entry_id] >2 {//How many times you can cancel
                    UPSMASH_CANCEL_COUNTER[entry_id] = 3;//How  many hits before disabling cancel
                    ENABLE_MULTIHIT_CANCEL[entry_id] = false; 
                }
                else {
                    ENABLE_MULTIHIT_CANCEL[entry_id] = true; 
                }

                //Reset
                if MOVEMENT_CANCEL[entry_id] {
                    if UPSMASH_CANCEL_COUNTER[entry_id] == 3 {
                        UPSMASH_CANCEL_COUNTER[entry_id] = 0;
                        ENABLE_MULTIHIT_CANCEL[entry_id] = false;
                    }    
                    MOVEMENT_CANCEL[entry_id] = false; 
                }
            }

            //Resets
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
                UPSMASH_CANCEL_COUNTER[entry_id] = 0;
                ENABLE_MULTIHIT_CANCEL[entry_id] = false; 
            }

        }
    }

pub fn install() {
    smashline::install_agent_frames!(younglink_opff);

}