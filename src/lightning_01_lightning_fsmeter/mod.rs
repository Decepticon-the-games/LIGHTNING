use skyline::nn::ro::LookupSymbol;
use smash_script::lua_args;
//use smash_script::notify_event_msc_cmd;
use smash::app::{lua_bind::{FighterManager, FighterInformation, *}, *};
use smash::app::smashball::is_training_mode;
use smash::lib::{lua_const::*};
use smash::lua2cpp::{L2CFighterCommon};
//use smash::lib::lua_const::*;
use smashline::*;
use smash::hash40;
use smash::phx::Hash40;
use crate::lightning_01_crimson_cancel::CRIMSON_CANCEL_TIMER;
use crate::lightning_01_lightning_mode::LIGHTNING;
use crate::lightning_01_ultrainstinct::SEC_SEN_STATE;

pub static mut FIGHTER_MANAGER_ADDR: usize = 0;
//pub static mut START_FS: [bool;9] = [false;9]; //Used for canceling taunt startup into FS
pub static mut DISABLE_FINAL : [bool; 8] = [false; 8];//Disable final smash/Special Button returns to neutral special
//pub static mut SUB_METER: [f32;9] = [0.0;9]; //Adds or subtracts from the meter value
//pub static mut FS_METER: [f32;9] = [0.0;9]; //Tracks meter
pub static mut FINAL_SMASH_BUTTON : [bool; 8] = [false; 8];


//Hoook final smash params to mauipulate meter (not give charge over time, decrease meter when using special mechanics, require some meter to do special mechanics)

    #[skyline::hook(offset=0x4e53c0)]
    pub unsafe fn hook_fs_meter(boma: u64,param_type: u64,param_hash: u64) -> f32 {
        let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
        //let owner_module_accessor = &mut *app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        //let fighter_kind = app::utility::get_kind(module_accessor);
        //let status_kind = StatusModule::status_kind(module_accessor);
        //let situation_kind = StatusModule::situation_kind(module_accessor);
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
                if CRIMSON_CANCEL_TIMER[entry_id] <= 120 && CRIMSON_CANCEL_TIMER[entry_id] >= 1 {//if crimson cancel
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
        //let fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
        //let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);

        if param_hash == hash40("charge_final_decrease_start_frame") || param_hash == hash40("charge_final_remove_frame") {

            if param_hash == hash40("charge_final_decrease_start_frame") {

                return 1; //1 FRAME TO IMMEDIATELY START DECREASING
   
            }
            if param_hash == hash40("charge_final_remove_frame") {

               return 600; //10 seconds
   
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
            //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            //let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 1);
            let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
            //let jump_guard_dash_upspecial_pressed = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0);
            //let frame = MotionModule::frame(module_accessor);
            let fsmeterfloat = WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHARGE_FINAL_GAUGE);
            let is_cpu = FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(fighter_manager,smash::app::FighterEntryID(entry_idi32)));

            let idles = (status_kind == *FIGHTER_STATUS_KIND_WAIT
            || status_kind == *FIGHTER_STATUS_KIND_FALL
            || status_kind == *FIGHTER_STATUS_KIND_FALL_AERIAL);

            let walks_runs_jumps_falls = (status_kind == *FIGHTER_STATUS_KIND_WALK
            || status_kind == *FIGHTER_STATUS_KIND_DASH
            || status_kind == *FIGHTER_STATUS_KIND_TURN_DASH
            || status_kind == *FIGHTER_STATUS_KIND_JUMP
            || status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL);

            
    //MISSING HERE, if not enough meter        
        if fsmeterfloat <= 20.0 {
        //    CAN_CRIMSON_CANCEL[entry_id] = false;
        //    SEC_SEN_STATE[entry_id] = false;
        //    SECRET_SENSATION[entry_id] = false;
        }


    //when meter fills up, remove fs effects on body
            if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE) {//FINAL SMASH STATE WILL NEVER ACTIVATE (????), not until button command
                
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
 

    //Turn meter on in training mode??

            if is_training_mode() {
                
                //MISSING HERE, turn on meter
            }
        }
    }
    

//----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------



//
pub fn install() {
    smashline::install_agent_frame_callbacks!(fs_meter);
    skyline::install_hook!(hook_fs_meter);
    skyline::install_hook!(meter_timer);
} 