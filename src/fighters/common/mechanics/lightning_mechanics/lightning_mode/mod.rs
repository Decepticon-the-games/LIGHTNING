use skyline::nn::ro::LookupSymbol;
use smash_script::lua_args;
use smash::app::{self, lua_bind::{FighterManager, FighterInformation, *}, sv_animcmd::*, *};
use smash::app::smashball::is_training_mode;
use smash::lib::{lua_const::*};
use smash::lua2cpp::{L2CFighterCommon};
use smashline::*;
use smash::hash40;
use smash::phx::{Hash40, Vector4f};
use smash_script::*;

use crate::fighters::common::mechanics::lightning_mechanics::vanish::ACTIVATE_VANISH;
use crate::fighters::common::mechanics::cancels::attack_cancels::ENABLE_MULTIHIT_CANCEL;
use crate::fighters::common::mechanics::cancels::motioncancels::AIRDODGE_PLUS;


pub static mut FIGHTER_MANAGER_ADDR: usize = 0;
pub static mut LIGHTNING: [bool; 8] = [false; 8];
pub static mut CAN_LIGHTNING: [bool; 8] = [true; 8];

pub static mut LIGHTNING_TIMER : [i32; 8] = [-1;  8];
pub static mut CAN_LIGHTNING_TEMP : [bool; 8] = [true; 8];
pub static mut LIGHTNING_BUTTON : [bool; 8] = [false; 8];
pub static mut LIGHTNING_EFFECTS: [bool; 8] = [false; 8];
pub static mut ONEFRAMEEFFECTS: [bool; 8] = [false; 8];

//WHEN METER GETS IMPLEMENTED, LIGHTNING METER GAGUE WILL BE BASED ON DAMAGE DEALT AND DAMAGE TAKEN (PERCENTAGE).
//EYES WILL TURN BLUE
//---------------------------------------------------------------------------------------------------------------- 

//BUTTON
#[fighter_frame_callback]
pub fn lightning_button(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        //println!("can_lightning: {}", CAN_LIGHTNING[entry_id]); 
        if LIGHTNING_TIMER[entry_id] == -1
        && CAN_LIGHTNING[entry_id] {

            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) { 
                
                LIGHTNING[entry_id] = true;
                
                lightning(fighter);
            }
        }
    }
}

//MECHANICS
pub fn lightning(fighter : &mut L2CFighterCommon) {
    unsafe{
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        
            lightning_effects(fighter);
            lightning_timer(fighter);
            //CAN_LIGHTNING_TEMP = CAN_LIGHTNING;         
            CAN_LIGHTNING[entry_id] = false;
            //ALL Attack move move a bit fasteer
                
            /*if moveset {
                
                if AttackModule::is_attack(fighter.module_accessor, 0, false) {
                    MotionModule::set_rate(fighter.module_accessor, 1.0);
                }
                else {
                    MotionModule::set_rate(fighter.module_accessor, 1.3);
                    //MotionModule::set_rate(fighter.module_accessor, 2.0);
                }

            } */

            //In STAMINA MODE, set damage multiplier 1.3x and heal
            if FighterUtil::is_hp_mode(fighter.module_accessor) {
                DamageModule::set_damage_mul(fighter.module_accessor, 1.3);
                DamageModule::heal(fighter.module_accessor, 1.3, 2);
            }

            //Vanish Mechanic enabled
                //ACTIVATE_VANISH[entry_id] = true;  
                
            //Mulihit Cancels enabled
            ENABLE_MULTIHIT_CANCEL[entry_id] = true;
    }
}

//VISUAL EFFECTS
pub unsafe fn lightning_effects(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);

    EffectModule::req_emit(fighter.module_accessor, Hash40::new("sys_aura_light"), 0);
    EffectModule::set_rgb(fighter.module_accessor, 0,0.0, 0.784, 1.0);
    EffectModule::req_emit(fighter.module_accessor, Hash40::new("sys_aura_light"), 0);  
    EffectModule::set_rgb(fighter.module_accessor, 0,0.0, 0.784, 1.0);
    macros::BURN_COLOR(fighter, 0.0, 0.784, 1.0, 0.7);
    if ! status_kind == *FIGHTER_STATUS_KIND_REBIRTH {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_dead"), Hash40::new("top"), 0, 9, -0.5, 0, 0, 0, 0.5, false);   
        macros::BURN_COLOR(fighter, 0.0, 0.784, 1.0, 0.7);        
    }
}
//TIMER 
pub fn lightning_timer(fighter : &mut L2CFighterCommon) {
    unsafe {
        LookupSymbol(&mut FIGHTER_MANAGER_ADDR, "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}".as_bytes().as_ptr());
        let fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let entry_idi32 = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let dead_count = FighterInformation::dead_count(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(entry_idi32)),0);
        let stock_count = FighterInformation::stock_count(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(entry_idi32)));
        
        println!("timer: {}", LIGHTNING_TIMER[entry_id]);
            
        if stock_count >=4 {
            LIGHTNING_TIMER [entry_id] = 1200; //20 seconds
        }

        if stock_count == 3 {
        LIGHTNING_TIMER [entry_id] = 1800; //30 seconds
        }
        if stock_count == 2 {
            LIGHTNING_TIMER [entry_id] = 2400; //40 seconds
        }
        if stock_count == 1 {
            LIGHTNING_TIMER [entry_id] = 3600; //1 minute
        }            
    }
}
//COUNTDOWN
#[fighter_frame_callback]
pub fn lightning_timer_countdown(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        

        //Countdown
        if LIGHTNING_TIMER[entry_id] >= 1 {
            if ! (status_kind == *FIGHTER_STATUS_KIND_REBIRTH || status_kind == *FIGHTER_STATUS_KIND_DEAD) {
                LIGHTNING_TIMER[entry_id] -=1; //Will halt countdown on death/respawn
            }
        }
        //STOP LIGHTNING
        if LIGHTNING_TIMER[entry_id] == 0 { // Only when the timer reaches 0 will u be unable to perform lightning again for the rest of the match
            //CAN_LIGHTNING = CAN_LIGHTNING_TEMP;
            lightning_disable(fighter);
        }    
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH && LIGHTNING[entry_id] && MotionModule::frame(fighter.module_accessor) == 1.0 {//callback effects once
            lightning_effects(fighter);
        }       
    }
}
//DISABLE
pub unsafe fn lightning_disable(fighter : &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    LIGHTNING_TIMER[entry_id] = -1;
    LIGHTNING[entry_id] = false;
    macros::BURN_COLOR_NORMAL(fighter);
    macros::COL_NORMAL(fighter);
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), true, true);   
    ModelModule::disable_gold_eye(fighter.module_accessor);	
    ENABLE_MULTIHIT_CANCEL[entry_id] = false;        
}

//RESETS
#[fighter_frame_callback]
pub fn lightning_resets(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        if smash::app::sv_information::is_ready_go() == false 
        {
            lightning_disable(fighter);
            CAN_LIGHTNING[entry_id] = true;
        }
    }
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(lightning_button, lightning_timer_countdown, lightning_resets);
}  

