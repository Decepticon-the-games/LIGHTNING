use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;

use smash_script::*;
//use smash::phx::Hash40;
//use smash::hash40;
use crate::lightning_01_vanish::VANISH_READY;
use crate::hooks::PROJECTILE_HIT;


pub static mut ATTACK_CANCEL : [bool; 8] = [false; 8];


#[fighter_frame_callback]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        //let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 1);
        //let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
        let frame = MotionModule::frame(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        //let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        
        


        
        //ATTACK CANCELS
        if ATTACK_CANCEL[entry_id] && ! WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STATUS) {
            if AttackModule::is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor) { 
                CancelModule::enable_cancel(fighter.module_accessor);
                
            }              
        }
        else  {
            ATTACK_CANCEL[entry_id] = false;
        }        

        

        //REMOVE INVINCIBILITY ON SHIELD BREAK 
        
        if status_kind == *FIGHTER_STATUS_KIND_SHIELD_BREAK_FALL {
            macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
        }

        //EASIER WAVEDASH CHAINS// 
        if status_kind == *FIGHTER_STATUS_KIND_LANDING && frame >10.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
            
        
        //REMOVE LANDING LAG
        if (status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR || status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL) && frame >1.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
        
        //AIRDASH
        if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
            if frame >= 4.0 {
                
                CancelModule::enable_cancel(fighter.module_accessor);
                
            }
        }
        
        // ROLL??
        if motion_kind== smash::hash40("escape_f") || motion_kind== smash::hash40("escape_b")  {

        }

        //SHIMMY??
        if status_kind == *FIGHTER_STATUS_KIND_DASH {
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 {
                //PostureModule::reverse_lr(fighter.module_accessor);
            }   
        }

        //DODGES ARE FASTER
        
        //Get airdodge back during free fall
        if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            //if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD != 0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor,*FIGHTER_STATUS_KIND_FALL,true); 
            //}   
        }
        
        //GRAB COMBOS
        if status_kind == *FIGHTER_STATUS_KIND_THROW && AttackModule::is_attack_occur(fighter.module_accessor) {
            
            if ! (fighter_kind == *FIGHTER_KIND_SHULK && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0) { //so shulk's monado arts don't break
                CancelModule::enable_cancel(fighter.module_accessor);
            }
            
        }
        //MOONWALKING
        let speed = smash::phx::Vector3f {x: -1.0, y: 0.0, z: 0.0 };
        let mut stick_x = ControlModule::get_stick_x(fighter.module_accessor);
        stick_x = stick_x * PostureModule::lr(fighter.module_accessor);

        if (motion_kind == smash::hash40("dash") || motion_kind == smash::hash40("turn_dash")) && MotionModule::frame(fighter.module_accessor) <= 3.0 && stick_x <= -0.3 {
            KineticModule::add_speed(fighter.module_accessor,  &speed);
        };
       

        //REWARD PARRIES WITH INVINCIBILITY

        if motion_kind== smash::hash40("just_shield") || motion_kind== smash::hash40("just_shield_off") {
            HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);    
            
        }

        

        //NO JAB CHAINS (CANCEL WITH JUMP/GRAB/DASH)
        if motion_kind== smash::hash40("attack_11")
        || motion_kind== smash::hash40("attack_12")
        || motion_kind== smash::hash40("attack_13")
        || motion_kind== smash::hash40("attack_14")
        || motion_kind== smash::hash40("attack_15")
        || motion_kind== smash::hash40("attack_16")
        || motion_kind== smash::hash40("attack_17")
        || motion_kind== smash::hash40("attack_18")
        || motion_kind== smash::hash40("attack_19")
        || motion_kind== smash::hash40("attack_100")
        || motion_kind== smash::hash40("attack_100_sub")
        || motion_kind== smash::hash40("attack_100_end")
        || motion_kind== smash::hash40("attack_110") {
            
            if AttackModule::is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor) {
                
                
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 {
                    CancelModule::enable_cancel(fighter.module_accessor); 
                }
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 {
                    if ! (fighter_kind == *FIGHTER_KIND_DEMON)  { //Kazuya is the only one that can't dash after jab
                        CancelModule::enable_cancel(fighter.module_accessor); 
                    }
                    
                }
                
            } 
        }
    }
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(once_per_fighter_frame);

}