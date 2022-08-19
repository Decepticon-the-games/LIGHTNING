use smash::app::*;
use smash::app::lua_bind::{self, FighterManager, *};
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;

use smash_script::*;
use smash::app::sv_animcmd::{self};
//use crate::lightning_02_up_special_callbacks::UP_SPECIAL_ANIMATION;
//use crate::lightning_02_up_special_callbacks::ENTRY_ID;

// CREATED BY PHAZOGANON, THANK YOU :)

// LIGHTNING_CANCEL_TIMER triggers faster motion rate for attacks and specials, infinite hit-cancel airdodges, removes final smash over neutral special//

//static mut LIGHTNING_CANCEL : [bool; 8] = [false;  8];
//static mut LIGHTNING_CANCEL_TIMER : [i32; 8] = [-1;  8];
//pub const FINAL_AURA_HASH: u64 = smash::hash40("sys_final_aura");

//pub static mut SPECIAL_N_FINAL_SMASH_METER : [bool; 8] = [false; 8];
pub static mut DISABLE_FINAL : [bool; 8] = [false; 8];


#[fighter_frame_callback]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let lua_state = fighter.lua_state_agent; 
        let status_kind = StatusModule::status_kind(module_accessor);
        let situation_kind = StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        

        //println!("something: {}", WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHARGE_FINAL_GAUGE));

        
        if //WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE)
        && FigtherManager::is_final(fighter_manager) { //IN THE FINAL SMASH AURA STATE...

            sv_animcmd::REMOVE_FINAL_SCREEN_EFFECT(fighter.lua_state_agent);
            
            //sv_animcmd::FT_REMOVE_FINAL_AURA(fighter.lua_state_agent);

            
            
            //Take Less Damage Knockback

            //Special Button returns to neutral special
            DISABLE_FINAL[entry_id] = true;
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {

                //StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, false);
                WorkModule::unable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL)
                
            }

            //Different buttons to activate final smash

            ///if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CATCH)
            ///&& ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD)
            //&& ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {

                //StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FINAL, false);

            //}       

                
            //AIRDODGE ON HIT EXCEPT UP SPECIAL OF ALL KINDS
    
            if situation_kind == *SITUATION_KIND_AIR {

                    
                if AttackModule::is_attack_occur(module_accessor) && ! SlowModule::is_slow(module_accessor) {

                    //AIRDODGE

                    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                    }
                }   

            }

                
            
            //ALL Attack move move a bit fasteer
            
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3
            || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3
            || status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3
            || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4
            || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4
            || status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4
            || status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR
            || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N
            || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
            || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
            || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                
                if AttackModule::is_attack(module_accessor, 0, false) {
                    MotionModule::set_rate(module_accessor, 1.0);
                }
                else {
                    MotionModule::set_rate(module_accessor, 1.3);
                }

            } 
            
        }
    }
}
    
    
pub fn install() {
    smashline::install_agent_frame_callbacks!(once_per_fighter_frame);
}