use super::*;
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

//WHEN METER GETS IMPLEMENTED, LIGHTNING METER GAGUE WILL BE BASED ON DAMAGE DEALT AND DAMAGE TAKEN (PERCENTAGE).
//EYES WILL TURN BLUE
//---------------------------------------------------------------------------------------------------------------- 

//BUTTON
#[fighter_frame_callback]
pub fn lightning_button(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        let hitlag = (SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) > 0 || StopModule::is_stop(fighter.module_accessor));
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        let idles =  (status_kind == *FIGHTER_STATUS_KIND_WAIT || status_kind == *FIGHTER_STATUS_KIND_JUMP || status_kind == *FIGHTER_STATUS_KIND_FALL);

        //if attack cancel and the meter is filled, can_ligtning is enabled. Burn color to lightning blue.

        if lightning_mode_conditions(fighter)
        {
            if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0 
            && (idles || is_after_hitlag(fighter))
            {
                LIGHTNING_BUTTON[entry_id] = true;

            }
            else {
                LIGHTNING_BUTTON[entry_id] = false;
            }
            ///cpu testing only////
            if smash::app::sv_information::is_ready_go() == true {
                //LIGHTNING_BUTTON[entry_id] = true;
            }
            //////////////////////
        }

        if LIGHTNING_BUTTON[entry_id] 
        {
            ON_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING[entry_id] = true;
            LIGHTNING_BUTTON[entry_id] = false;
        }


    }
}
pub unsafe fn lightning_mode_conditions(fighter : &mut L2CFighterCommon) -> bool {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        LIGHTNING_TIMER[entry_id] == -1
        && CAN_LIGHTNING[entry_id]
    }
}
//MECHANICS
#[fighter_frame_callback]
pub fn lightning(fighter : &mut L2CFighterCommon) {
    unsafe{
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

            if LIGHTNING_PRE[entry_id] {
                lightning_effects(fighter);
                lightning_timer(fighter);   
                LIGHTNING_PRE[entry_id] = false;              
            }

            if ON_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING[entry_id] {
                //initial settings, works as a pre/init, runs once
                LIGHTNING_PRE[entry_id] = true; 
                 
                IS_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING[entry_id] = true;
                ON_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING[entry_id] = false;
            }
            
            if IS_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING[entry_id] {
                //every attack produces an effect
                if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
                    let pos = smash::phx::Vector3f {x:AttackModule::pos_x_2(fighter.module_accessor),y:AttackModule::pos_y(fighter.module_accessor),z:0.0};
                    let zero = smash::phx::Vector3f {x:0.0 ,y:0.0,z:0.0};
                    EffectModule::req(fighter.module_accessor, smash::phx::Hash40::new("sys_hit_elec"), &pos, &zero, 0.5, 0, 0, false, 0);    
                    macros::LAST_EFFECT_SET_COLOR(fighter,0.0, 0.784, 1.0,);          
                } 
                macros::BURN_COLOR(fighter, 0.0, 0.784, 1.0, 0.7);
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
                    
                //Mulihit Cancels enabled(in character opff)                            
            }       

    }
}

//VISUAL EFFECTS
pub unsafe fn lightning_effects(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    EffectModule::req_emit(fighter.module_accessor, Hash40::new("sys_aura_light"), 0);
    macros::LAST_EFFECT_SET_COLOR(fighter,0.0, 0.784, 1.0,);
    //EffectModule::set_rgb(fighter.module_accessor, 0,0.0, 0.784, 1.0);
    EffectModule::req_emit(fighter.module_accessor, Hash40::new("sys_aura_light"), 0); 
    macros::LAST_EFFECT_SET_COLOR(fighter,0.0, 0.784, 1.0,); 
    //ffectModule::set_rgb(fighter.module_accessor, 0,0.0, 0.784, 1.0);
    ModelModule::enable_gold_eye(fighter.module_accessor);	 
    //ModelModule::set_color_rgb(fighter.module_accessor, 0.0, 0.784, 1.0, MODEL_COLOR_TYPE::MODEL_COLOR_TYPE_NORMAL);
    WorkModule::set_float(fighter.module_accessor, 50.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHARGE_FINAL_ADD_DAMAGE)
    /*if ATTACK_CANCEL[entry_id] {

    }*/
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
        
        //println!("timer: {}", LIGHTNING_TIMER[entry_id]);
            
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
        
        //println!("lightningtimer: {}", LIGHTNING_TIMER[entry_id] );
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
    }
}
//DISABLE
pub unsafe fn lightning_disable(fighter : &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    OFF_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING[entry_id] = true;
    
    if OFF_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING[entry_id] {
        LIGHTNING_TIMER[entry_id] = -1;
        IS_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING[entry_id] = false;
        macros::BURN_COLOR_NORMAL(fighter);
        macros::COL_NORMAL(fighter);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), true, true);   
        ModelModule::disable_gold_eye(fighter.module_accessor);	
        OFF_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING[entry_id] = false;
    }   
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
    smashline::install_agent_frame_callbacks!(lightning, lightning_button, lightning_timer_countdown, lightning_resets);
}  

