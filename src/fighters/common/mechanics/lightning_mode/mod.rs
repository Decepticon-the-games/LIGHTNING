use skyline::nn::ro::LookupSymbol;
use smash_script::lua_args;
use smash::app::{self, lua_bind::{FighterManager, FighterInformation, *}, *};
use smash::app::smashball::is_training_mode;
use smash::lib::{lua_const::*};
use smash::lua2cpp::{L2CFighterCommon};
use smashline::*;
use smash::hash40;
use smash::phx::Hash40;
use smash_script::*;

use crate::fighters::common::mechanics::vanish::ACTIVATE_VANISH;
//use crate::lightning_02_up_special_callbacks::ENTRY_ID;

pub static mut FIGHTER_MANAGER_ADDR: usize = 0;
pub static mut LIGHTNING: [bool; 8] = [false; 8];
pub static mut CAN_LIGHTNING: [bool; 8] = [true; 8];
//pub static mut LIGHTNING_STOP: [bool; 8] = [false; 8];
pub static mut LIGHTNING_TIMER : [i32; 8] = [-1;  8];
pub static mut CAN_LIGHTNING_TEMP : [bool; 8] = [true; 8];
pub static mut LIGHTNING_BUTTON : [bool; 8] = [false; 8];
pub static mut LIGHTNING_EFFECTS: [bool; 8] = [false; 8];
// CREATED BY PHAZOGANON, THANK YOU :)

// LIGHTNING_CANCEL_TIMER triggers faster motion rate for attacks and specials, 

//LIGHTNING
#[fighter_frame_callback]
pub fn lightning(fighter : &mut L2CFighterCommon) {
    unsafe {
        LookupSymbol(&mut FIGHTER_MANAGER_ADDR, "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}".as_bytes().as_ptr());
        let fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let status_kind = StatusModule::status_kind(module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        ////let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 1);
        //let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
        ////let jump_guard_dash_upspecial_pressed = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0);
        //let frame = MotionModule::frame(module_accessor);
        //let lua_state = fighter.lua_state_agent;
        let entry_idi32 = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let dead_count = FighterInformation::dead_count(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(entry_idi32)),0);
        let moveset = (status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR
        || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N
        || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
        || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
        || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW);

//Activate Lightning Mode (during meter)

        if LIGHTNING_TIMER[entry_id] == -1
        && CAN_LIGHTNING[entry_id] {
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
            //&& ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            //&& ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
            // If the fighte is actionable
            {//if button command pressed
                
                LIGHTNING_BUTTON[entry_id] = true;
            }
        }


        if LIGHTNING_BUTTON[entry_id] {
            LIGHTNING[entry_id] = true; 
            LIGHTNING_EFFECTS[entry_id] = true;
            LIGHTNING_BUTTON[entry_id] = false;
        }

        if LIGHTNING_EFFECTS[entry_id] {

            EffectModule::req_emit(module_accessor, Hash40::new("sys_damage_aura"), 0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
            ModelModule::enable_gold_eye(module_accessor);	
            //EffectModule::req_emit(module_accessor, Hash40::new("sys_final_aura2"), 1);
            macros::LAST_PARTICLE_SET_COLOR(fighter, 0.0, 0.851, 1.0);

            if dead_count == 0 {
                LIGHTNING_TIMER [entry_id] = 1200; //20 seconds
            }
            //if status_kind == *FIGHTER_STATUS_KIND_REBIRTH {

            if dead_count == 1 {
            LIGHTNING_TIMER [entry_id] = 1800; //30 seconds
            }
            if dead_count == 2 {
                LIGHTNING_TIMER [entry_id] = 2400; //40 seconds
            }
            if dead_count >= 3 {
                LIGHTNING_TIMER [entry_id] = 3600; //1 minute
            }   
            //}
            if is_training_mode() {
        
                LIGHTNING_TIMER [entry_id] = 999999; //1 minute
            }      
            LIGHTNING_EFFECTS[entry_id] = false; 
        }

        //Countdown
        if LIGHTNING_TIMER[entry_id] >= 1 {
        
            LIGHTNING_TIMER[entry_id] -=1;
        }  
        
        
//In Lightning mode...

        if LIGHTNING[entry_id] { 

            CAN_LIGHTNING_TEMP = CAN_LIGHTNING;
            CAN_LIGHTNING[entry_id] = false;
            //MISSING HERE, remove the hightened probability of cpus using final smash, they think theyre pressing final smash but itll come out as neutral b            
          

        //ALL Attack move move a bit fasteer
                
            if moveset {
                
                if AttackModule::is_attack(module_accessor, 0, false) {
                    MotionModule::set_rate(module_accessor, 1.0);
                }
                else {
                    MotionModule::set_rate(module_accessor, 1.3);
                    //MotionModule::set_rate(module_accessor, 2.0);
                }

            } 
        //In STAMINA MODE, set damage multiplier 1.3x and heal
            if FighterUtil::is_hp_mode(module_accessor) {
                DamageModule::set_damage_mul(module_accessor, 1.3);
                DamageModule::heal(module_accessor, 1.3, 2);
            }
            CAN_LIGHTNING[entry_id] = false;
        
        
        //Vanish Mechanic enabled
            //ACTIVATE_VANISH[entry_id] = true;       
            
        }



//STOP LIGHTNING
        if LIGHTNING_TIMER[entry_id] == 0 {
            LIGHTNING_TIMER[entry_id] = -1;
            LIGHTNING[entry_id] = false;
            sv_animcmd::FT_REMOVE_FINAL_AURA(fighter.lua_state_agent);   
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_final_aura2"), true, true); 
            MotionModule::set_rate(module_accessor, 1.0);    
            ModelModule::disable_gold_eye(module_accessor);	

            CAN_LIGHTNING = CAN_LIGHTNING_TEMP;
        }

//RESET EACH MATCH
        if smash::app::sv_information::is_ready_go() == false  {
            CAN_LIGHTNING[entry_id] = true;
            LIGHTNING_TIMER[entry_id] = -1;
            LIGHTNING[entry_id] = false;
            sv_animcmd::FT_REMOVE_FINAL_AURA(fighter.lua_state_agent);   
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_final_aura2"), true, true); 
            MotionModule::set_rate(module_accessor, 1.0);    
            ModelModule::disable_gold_eye(module_accessor);
        }
      
        




    }
    
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(lightning);
} 