use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;

use smash_script::*;
use crate::fighters::common::mechanics::vanish::{VANISH_READY, CANCEL_INTO_VANISH};
use crate::fighters::common::function_hooks::cross_cancel_vanish::PROJECTILE_HIT;

pub static mut ATTACK_CANCEL : [bool; 8] = [false; 8];
pub static mut ENABLE_ATTACK_CANCEL : [bool; 8] = [false; 8];
pub static mut ATTACK_CANCEL_COUNTER : [bool; 8] = [false; 8];
pub static mut ATTACK_CANCEL_COUNT : [i32; 8] = [0; 8];
pub static mut ENABLE_MULTIHIT_CANCEL : [bool; 8] = [false; 8];
pub static mut MOVEMENT_CANCEL : [bool; 8] = [true; 8];

#[fighter_frame_callback]
pub fn attack_cancels(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);     
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        let movement_cancel = (ControlModule::is_enable_flick_jump(fighter.module_accessor) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0) 
        || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 
        || (((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0) && situation_kind == *SITUATION_KIND_GROUND) 
        || (((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0) && situation_kind == *SITUATION_KIND_AIR);

        let idles = (status_kind == *FIGHTER_STATUS_KIND_WAIT
        || status_kind == *FIGHTER_STATUS_KIND_FALL
        || status_kind == *FIGHTER_STATUS_KIND_FALL_AERIAL);

        let walks_runs_jumps_falls = (status_kind == *FIGHTER_STATUS_KIND_WALK
        || status_kind == *FIGHTER_STATUS_KIND_DASH
        || status_kind == *FIGHTER_STATUS_KIND_TURN_DASH
        || status_kind == *FIGHTER_STATUS_KIND_JUMP
        || status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL);

        //if entry_id == 0 {
            //println!("occur: {}", AttackModule::is_attack_occur(fighter.module_accessor));
            //println!("is: {}", AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL));
            //println!("has: {}", AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL));
            //println!("en-multi: {}", ENABLE_MULTIHIT_CANCEL[entry_id]);
            //println!("en-attack: {}", ENABLE_ATTACK_CANCEL[entry_id]);
        //}
        

        if idles || walks_runs_jumps_falls{
            ENABLE_ATTACK_CANCEL[entry_id] = false;
            ENABLE_MULTIHIT_CANCEL[entry_id] = false;
        }

        //NON CANCELLABLE MOVES
        /*if (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK) 
        || (status_kind == *FIGHTER_STATUS_KIND_FINAL)
        || (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && ! fighter_kind == *FIGHTER_KIND_DONKEY)
        || (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        || (status_kind == *FIGHTER_STATUS_KIND_THROW) {
            
            ENABLE_ATTACK_CANCEL[entry_id] = false;
        }*/    
//JAB CANCELS, CAN ONLY CANCEL TWICE

        /*static mut JAB_CANCEL_COUNT : [bool; 8] = [false; 8];
        static mut JAB_CANCEL_COUNTER : [i32; 8] = [0; 8];
*
        if (motion_kind == smash::hash40("attack_11") 
        || motion_kind == smash::hash40("attack_12")
        || motion_kind == smash::hash40("attack_13"))
        && fighter_kind != *FIGHTER_KIND_DEMON {
            if AttackModule::is_attack_occur(fighter.module_accessor) {
                //Counter 
                if JAB_CANCEL_COUNT[entry_id] == false {
                    JAB_CANCEL_COUNTER[entry_id] +=1;
                    JAB_CANCEL_COUNT[entry_id] = true; 
                }

            }
            else {
                JAB_CANCEL_COUNT[entry_id] = false;
            }
            //Disable cancel
            if JAB_CANCEL_COUNTER[entry_id] >2 {//How many times you can cancel
                JAB_CANCEL_COUNTER[entry_id] = 3;//How  many hits before disabling cancel
                ENABLE_MULTIHIT_CANCEL[entry_id] = false; 
            }
            else {
                if ENABLE_MULTIHIT_CANCEL[entry_id] {
    ENABLE_ATTACK_CANCEL[entry_id] = true;
}
            }
            //Reset
            if movement_cancel {
                if JAB_CANCEL_COUNTER[entry_id] == 2 {
                    JAB_CANCEL_COUNTER[entry_id] = 0;
                    if ENABLE_MULTIHIT_CANCEL[entry_id] {
    ENABLE_ATTACK_CANCEL[entry_id] = true;
}
                }    
            }
        }*/

//ENABLE ATTACK CANCELS// this instance prevents cancelling more than a certain amount without first moving
        
        if ENABLE_ATTACK_CANCEL[entry_id]
        {
            if AttackModule::is_attack_occur(fighter.module_accessor) {
                if ATTACK_CANCEL_COUNTER[entry_id] == false {
                    ATTACK_CANCEL_COUNT[entry_id] +=1;
                    ATTACK_CANCEL_COUNTER[entry_id] = true;
                    //ENABLE_ATTACK_CANCEL[entry_id] = false; 
                }
                //the amount of times u can cancel in a row
                if ATTACK_CANCEL_COUNT[entry_id] > 2 {
                    ATTACK_CANCEL_COUNT[entry_id] = 3;
                    ATTACK_CANCEL[entry_id] = false;       
                } 
                else {
                    ATTACK_CANCEL[entry_id] = true;
                }            
            }
            else {
                ATTACK_CANCEL_COUNTER[entry_id] = false; 
            }
            //moovement options, always reset attack cancel
            if movement_cancel {
                ATTACK_CANCEL_COUNT[entry_id] = 0; 
            }  
            //ENABLE_ATTACK_CANCEL[entry_id] = false;
        }  
        /*else if ENABLE_MULTIHIT_CANCEL[entry_id] {
            //WRITE SYSTEM HERE
        }
        */
        

//RESETS
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
            ATTACK_CANCEL_COUNT[entry_id] = 0;
            ENABLE_ATTACK_CANCEL[entry_id] = false;

        } 

//ATTACK CANCELS

        if ATTACK_CANCEL[entry_id] 
        && ! (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STATUS) || status_kind == *FIGHTER_STATUS_KIND_FINAL) {
            
            if AttackModule::is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor) { 
                CancelModule::enable_cancel(fighter.module_accessor);
                ENABLE_ATTACK_CANCEL[entry_id] = false;
                ATTACK_CANCEL[entry_id] = false; 
            }
        
            
            
            

            //VANISH
            if AttackModule::is_attack_occur(fighter.module_accessor) || PROJECTILE_HIT[entry_id] {
                CANCEL_INTO_VANISH[entry_id] = true;
            }                          
            //else {
            //    CANCEL_INTO_VANISH[entry_id] = false;
            //}
                    
        }
        else  {
            ATTACK_CANCEL[entry_id] = false;
        } 
    }
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(attack_cancels);
}