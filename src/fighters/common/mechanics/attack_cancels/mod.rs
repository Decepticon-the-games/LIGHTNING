use super::*;
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

#[fighter_frame_callback]
pub fn attack_cancels(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        let movement_cancel = (ControlModule::is_enable_flick_jump(fighter.module_accessor) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0) 
        || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 
        || (((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0) && situation_kind == *SITUATION_KIND_GROUND) 
        || (((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0) && situation_kind == *SITUATION_KIND_AIR);

        //println!("enable: {}", ENABLE_ATTACK_CANCEL[entry_id]);
        //ENABLE ATTACK CANCELS// this instance prevents cancelling more than a certain amount without first moving

        if ENABLE_ATTACK_CANCEL[entry_id] {
            
            if AttackModule::is_attack_occur(fighter.module_accessor) {
                if ATTACK_CANCEL_COUNTER[entry_id] == false {
                    ATTACK_CANCEL_COUNT[entry_id] +=1;
                    ATTACK_CANCEL_COUNTER[entry_id] = true;
                }              
            }
            else {
                ATTACK_CANCEL_COUNTER[entry_id] = false;
            }

            //the amount of times u can cancel in a row
            if ATTACK_CANCEL_COUNT[entry_id] >= 3 {
                ATTACK_CANCEL_COUNT[entry_id] = 3;
                ATTACK_CANCEL[entry_id] = false;       
            }
            else {
                ATTACK_CANCEL[entry_id] = true;
            }
            
            //moovement options, always reset attack cancel
            if movement_cancel {
                ATTACK_CANCEL_COUNT[entry_id] = 0; 
            }
            ENABLE_ATTACK_CANCEL[entry_id] = false;        
        }  
        if (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK) 
        || (status_kind == *FIGHTER_STATUS_KIND_FINAL)
        || (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
        || (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        || (status_kind == *FIGHTER_STATUS_KIND_THROW) {
            
            ENABLE_ATTACK_CANCEL[entry_id] = false;
        }   
        else {
            ENABLE_ATTACK_CANCEL[entry_id] = true;
        }         

        //RESETS
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
            ATTACK_CANCEL_COUNT[entry_id] = 0;
        } 

        //ATTACK CANCELS

        if ATTACK_CANCEL[entry_id] 
        && ! (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STATUS) || status_kind == *FIGHTER_STATUS_KIND_FINAL) {
            
            if AttackModule::is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor) { 
                CancelModule::enable_cancel(fighter.module_accessor);
            }
            
            

            //VANISH
            if AttackModule::is_attack_occur(fighter.module_accessor) || PROJECTILE_HIT[entry_id] {
                CANCEL_INTO_VANISH[entry_id] = true;
            }                          
            else{
                CANCEL_INTO_VANISH[entry_id] = false;
            }
            ATTACK_CANCEL[entry_id] = false;         
        }
        else  {
            ATTACK_CANCEL[entry_id] = false;
        } 
    }
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(attack_cancels);
}