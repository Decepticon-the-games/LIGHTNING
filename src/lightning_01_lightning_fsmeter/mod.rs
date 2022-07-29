use skyline::nn::ro::LookupSymbol;
use smash_script::lua_args;
use smash_script::notify_event_msc_cmd;
use smash::app::{self, sv_module_access::*, lua_bind::{self, FighterManager, FighterInformation, *}, sv_math, smashball, *};
use smash::app::smashball::is_training_mode;
use smash::lib::{lua_const::{self, *}, L2CValueType::*, L2CValueType, L2CAgent, L2CValue, L2CTable, L2CTable_meta, L2CInnerFunctionBase, L2CValueInner};
use smash::lua2cpp::{self, *};
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::lib::lua_const::*;
use smashline::*;
use smash::hash40;
use smash::phx::Hash40;
use smash_script::*;
use crate::lightning_01_crimson_cancel::CRIMSON_CANCELLING;
use crate::lightning_01_crimson_cancel::CAN_CRIMSON_CANCEL;
use crate::lightning_01_ultrainstinct_vanish::SEC_SEN_STATE;

pub static mut FIGHTER_MANAGER_ADDR: usize = 0;
pub static mut START_FS: [bool;9] = [false;9]; //Used for canceling taunt startup into FS
pub static mut DISABLE_FINAL : [bool; 8] = [false; 8];//Disable final smash/Special Button returns to neutral special
pub static mut LIGHTNING: [bool; 8] = [false; 8];
static mut CAN_LIGHTNING: [bool; 8] = [true; 8];
static mut LIGHTNING_STOP: [bool; 8] = [false; 8];
static mut LIGHTNING_TIMER : [i32; 8] = [-1;  8];
static mut CAN_LIGHTNING_TEMP : [bool; 8] = [true; 8];
pub static mut SUB_METER: [f32;9] = [0.0;9]; //Adds or subtracts from the meter value
pub static mut FS_METER: [f32;9] = [0.0;9]; //Tracks meter
pub const FINAL_AURA_HASH: u64 = smash::hash40("sys_final_aura");
pub static mut AURA_VECTOR: smash::phx::Vector3f = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};

#[allow(unused_unsafe)]

pub unsafe fn get_player_number(module_accessor:  &mut app::BattleObjectModuleAccessor) -> usize {
    let mut player_number = 8;
    if app::utility::get_kind(module_accessor) == *WEAPON_KIND_PTRAINER_PTRAINER {
        player_number = WorkModule::get_int(module_accessor, *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_INT_FIGHTER_ENTRY_ID) as usize; //PT plays by different rules
    }
    else if app::utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        player_number = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
    else {
        let mut owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        while app::utility::get_category(owner_module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER { //Keep checking the owner of the boma we're working with until we've hit a boma that belongs to a fighter
            owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        }
        player_number = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
    return player_number;
}

//Hoook final smash params to mauipulate meter (not give charge over time, decrease meter when using special mechanics, require some meter to do special mechanics)

    #[skyline::hook(offset=0x4e53c0)]
    pub unsafe fn hook_fs_meter(boma: u64,param_type: u64,param_hash: u64) -> f32 {
        let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
        let owner_module_accessor = &mut *app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let fighter_kind = app::utility::get_kind(module_accessor);
        let status_kind = StatusModule::status_kind(module_accessor);
        let situation_kind = StatusModule::situation_kind(module_accessor);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        if param_hash == hash40("charge_final_add_gauge_by_time") 
        || param_hash == hash40("charge_final_add_gauge_by_attack") 
        || param_hash == 0x2d073b8b11u64 // Meter per damage taken
        || param_hash == 0x2212ae242au64 
        || param_hash == 0x222ea31b73u64
        {

            if param_hash == 0x2d073b8b11u64 { // Meter per damage taken

                if LIGHTNING [entry_id] {
                    return 1.5;
                }
                else if SEC_SEN_STATE[entry_id] {// if cross cancel (because cross cancel still takes damage)
                    return -7.0; //take away some meter
                }  
                else {
                    return 0.9;
                }
            }
            

            if param_hash == hash40("charge_final_add_gauge_by_time") { //param charge_final_add_gauge_by_time to not give charge over time
                if CRIMSON_CANCELLING[entry_id] <= 120 && CRIMSON_CANCELLING[entry_id] >= 1 {//if crimson cancel
                    return -17.0;//take away some meter
                }
                else  {
                   return 0.0; //time to not give charge over time
                }
                
            }   
            

            if param_hash == 0x2212ae242au64 || param_hash == 0x222ea31b73u64 { //pulls the meter value when the meter is 100% full
                return 100.0; 
            }
        }
        original!()(boma, param_type, param_hash)  
    }
    

//Hook FS meter timer to activate when LIGHTNING is activated. Change the duration of full meter to essentially infinity, and LIGHTNING to 60 seconds.
    #[skyline::hook(offset=0x4e5380)]
    pub unsafe fn meter_timer(boma: u64,param_type: u64,param_hash: u64) -> i32 {
        LookupSymbol(&mut FIGHTER_MANAGER_ADDR, "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}".as_bytes().as_ptr());
        let fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
        let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
        let fighter_kind = app::utility::get_kind(module_accessor);
        let status_kind = StatusModule::status_kind(module_accessor);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let entry_idi32 = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        //let dead_count = FighterInformation::dead_count(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(entry_idi32)),0);

        if param_hash == hash40("charge_final_decrease_start_frame") || param_hash == hash40("charge_final_remove_frame") {

            if param_hash == hash40("charge_final_decrease_start_frame") {
                //if LIGHTNING [entry_id] {
                    return 1; //1 FRAME TO IMMEDIATELY START DECREASING
                //}
                //else{
                //   return 9999999; //essentially infinite 
                //}
                 
            }
            if param_hash == hash40("charge_final_remove_frame") {
                //if dead_count == 0 {
                    return 600; //10 seconds
                //}
                //if dead_count == 1 {
                //    return 2400; //40 seconds
                //}
                //if dead_count == 2 {
                //    return 3600; //60 seconds
                //}
                //if dead_count == 3 {
                //    return 99999999; //infinite
                //}      
            }
        }
        
        original!()(boma, param_type, param_hash)
    }
//
//FS METETR
    #[fighter_frame_callback]
    pub fn fs_meter(fighter : &mut L2CFighterCommon) {
        unsafe {
            LookupSymbol(&mut FIGHTER_MANAGER_ADDR, "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}".as_bytes().as_ptr());
            let fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
            let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let entry_idi32 = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
            let fighter_kind = utility::get_kind(module_accessor);
            let motion_kind = MotionModule::motion_kind(module_accessor);
            let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
            let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 1);
            let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
            let jump_guard_dash_upspecial_pressed = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0);
            let frame = MotionModule::frame(module_accessor);
            let fsmeterfloat = WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHARGE_FINAL_GAUGE);
            let is_cpu = FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(entry_idi32)));

            if fighter_kind == *FIGHTER_KIND_GANON {
                println!("something: {}", WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHARGE_FINAL_GAUGE));
            }
            
    //MISSING HERE, if not enough meter        
        if fsmeterfloat <= 20.0 {
        //    CAN_CRIMSON_CANCEL[entry_id] = false;
        //    SEC_SEN_STATE[entry_id] = false;
        //    SECRET_SENSATION[entry_id] = false;
        }


    //when meter fills up, remove fs effects on body
            if FS_METER[get_player_number(module_accessor)] >= 100.0 || WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE) {//FINAL SMASH STATE WILL NEVER ACTIVATE (????), not until button command
                
                fighter.clear_lua_stack();
                lua_args!(fighter, 0);                
                sv_animcmd::REMOVE_FINAL_SCREEN_EFFECT(fighter.lua_state_agent);
                fighter.clear_lua_stack();            
                sv_animcmd::FT_REMOVE_FINAL_AURA(fighter.lua_state_agent);    
                if motion_kind != hash40("just_shield_off") {
                    ModelModule::disable_gold_eye(module_accessor);	
                                    
                } //MISSING HERE, turn off auto fs state
            }
    //Different buttons to activate final smash, instead of Neutral B
            if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE) {
                
                
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
                    if is_cpu {
                        DISABLE_FINAL[entry_id] = false;
                    }
                    else {
                        DISABLE_FINAL[entry_id] = true;
                    }
                    

                }
                else {
                  DISABLE_FINAL[entry_id] = false;  
                }

            
                //if FighterUtil::is_hp_mode(module_accessor) {
                    if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) //button command
                    {
                        
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FINAL, false);
                    }
                //}

            }  

    
            
            


    //Turn meter on in training mode??

            if is_training_mode() {
                
                //MISSING HERE, turn on meter
            }
        }
    
    //Final smashes reduced knockback
    }
    

//----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
//LIGHTNING
    #[fighter_frame_callback]
    pub fn lightning(fighter : &mut L2CFighterCommon) {
        unsafe {
            LookupSymbol(&mut FIGHTER_MANAGER_ADDR, "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}".as_bytes().as_ptr());
            let fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
            let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let motion_kind = MotionModule::motion_kind(module_accessor);
            let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
            let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 1);
            let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
            let jump_guard_dash_upspecial_pressed = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0);
            let frame = MotionModule::frame(module_accessor);
            let lua_state = fighter.lua_state_agent;
            let entry_idi32 = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
            let dead_count = FighterInformation::dead_count(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(entry_idi32)),0);

    //Activate Lightning Mode (during meter)

            if LIGHTNING_TIMER[entry_id] == -1
            && CAN_LIGHTNING[entry_id] {
                if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
                //&& WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE) 
                {//if button command pressed
                    
                    LIGHTNING[entry_id] = true; 
                    EffectModule::req_emit(module_accessor, Hash40::new("sys_final_aura2"), 0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
                    ModelModule::enable_gold_eye(module_accessor);	
                    //EffectModule::req_emit(module_accessor, Hash40::new("sys_final_aura2"), 1);
                    //macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);

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
                }
            }
             


            if LIGHTNING_TIMER[entry_id] >= 1 {
            
                LIGHTNING_TIMER[entry_id] -=1;
            }
    //STOP LIGHTNING
            if LIGHTNING_TIMER[entry_id] == 0 {
                //LIGHTNING_STOP[entry_id] = true;
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
            }
          
            
    //In Lightning mode...

            if LIGHTNING[entry_id] { 

                CAN_LIGHTNING_TEMP = CAN_LIGHTNING;
                CAN_LIGHTNING[entry_id] = false;

                //AURA ONLY//
                    //on evevry bone
                        //macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1.0, false, 0.7);
                        //macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
                        //macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1.0, false, 0.7);
                        //macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
                        //macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1.0, false, 0.7);
                        //macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
                        //macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1.0, false, 0.7);
                        //macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
                        //macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("legl"), 0, 0, 0, 0, 0, 0, 1.0, false, 0.7);
                        //macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
                        //macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("legr"), 0, 0, 0, 0, 0, 0, 1.0, false, 0.7);
                        //macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
                        //macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.0, false, 0.7);
                        //macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
                        //macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.0, false, 0.7);
                        //macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
                        //macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 1.0, false, 0.7);
                        //macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
                        //macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("shoulderr"), 0, 0, 0, 0, 0, 0, 1.0, false, 0.7);
                        //macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
                        //macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.0, false, 0.7);
                        //macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
                        //macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new_raw("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1.5, false, 0.7);
                        //macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
                        //macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1.0, false, 0.7);
                        //macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);

                    //emit


                //MISSING HERE, remove the hightened probability of cpus using final smash, they think theyre pressing final smash but itll come out as neutral b            
            
        
            //AIRDODGE ON HIT reset

                if situation_kind == *SITUATION_KIND_AIR && AttackModule::is_attack_occur(module_accessor) && ! SlowModule::is_slow(module_accessor) {
                                                          
                    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
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
                        //MotionModule::set_rate(module_accessor, 2.0);
                    }

                } 
            //In STAMINA MODE, set damage multiplier 1.3x and heal
                if FighterUtil::is_hp_mode(module_accessor) {
                    DamageModule::set_damage_mul(module_accessor, 1.3);
                    DamageModule::heal(module_accessor, 1.3, 2);
                }
            
            }
            



        }
        
    }


//
pub fn install() {
    smashline::install_agent_frame_callbacks!(fs_meter);
    smashline::install_agent_frame_callbacks!(lightning);
    skyline::install_hook!(hook_fs_meter);
    skyline::install_hook!(meter_timer);
} 