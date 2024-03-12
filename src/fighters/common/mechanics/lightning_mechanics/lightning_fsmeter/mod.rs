use super::*;
use skyline::nn::ro::LookupSymbol;
use smash_script::lua_args;

use crate::fighters::common::mechanics::lightning_mechanics::crimson_cancel::FIGHTER_INSTANCE_WORK_ID_FLAG_CRIMSON_CANCEL;
use crate::fighters::common::mechanics::lightning_mechanics::lightning_mode::LIGHTNING_BUTTON;
use crate::fighters::common::mechanics::lightning_mechanics::ultrainstinct::SEC_SEN_STATE;

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

    if param_type == hash40("common") 
    {

        if param_hash == hash40("charge_final_add_gauge_by_damage_mul") { // Meter per damage taken

            if IS_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING [entry_id] {
                //return 1.5;
            }
            else if SEC_SEN_STATE[entry_id] {// if cross cancel (because cross cancel still takes damage)
                //return -7.0; //take away some meter
            }  
        }
        

        if param_hash == hash40("charge_final_add_gauge_by_time") { //param charge_final_add_gauge_by_time to not give charge over time
            if FIGHTER_INSTANCE_WORK_ID_FLAG_CRIMSON_CANCEL[entry_id] {//if crimson cancel
                //return -17.0;//take away some meter
            }
            else if FIGHTER_STATUS_KIND_VANISH[entry_id] {//if crimson cancel
                //return -33.0;//take away some meter
            }
            else  {
                return 0.0; //time to not give charge over time
            }
            
        }  
        if param_hash == hash40("charge_final_add_gauge_by_attack") {
            return 0.3;
        }


        if param_hash == hash40("charge_final_add_gauge_by_time_mul")
        || param_hash == hash40("charge_final_add_gauge_by_attack_mul") 
        || param_hash == hash40("charge_final_add_gauge_by_damage_mul")
        {
            return 1.0;
        } 

        if param_hash == 0x2873df4aedu64 { //Multiplier for player count
            return 1.0;
        }
        if param_hash == 0x2d073b8b11u64 { //charge_final_add_gauge_by_damage??
            return 0.6;
        }

        if param_hash == 0x2d2bae85eeu64 { // 70, unknown
            //return 0.0;
        }

        if param_hash == 0x2b0de64f73u64 { // 0.9, unknown, cpus push out much much more aggression, more offensive
            //return 100.0;
        }

        if param_hash == 0x223f5d53c2u64 { // 0.5, unknown, offensive?
            //return 50.0;
        }
        if param_hash == 0x2203506c9bu64 { //1.5, unknown
            //return 0.0;
        }
        if param_hash == 0x232ccb683fu64 {//70, unknown
            //return 0.0;
        }
        if param_hash == 0x2212ae242au64 { //When the meter is 100% full and u get KO'd,before the meter starts decreasing, this is the value the meter resets to.
            return 100.0; 
        }
        if param_hash == 0x222ea31b73u64 { //When the meter is 100% full and u get KO'd while the meter is decreasing, this is the value the meter resets to.
            return 100.0;
        }
        if param_hash == 0x3346e488ffu64 {
            //return 0.0;
        }
        if param_hash == 0x337ae9b7a6u64 {
            //return 0.0;
        }

        if param_hash == 0x33fe2103c5u64 {
            //return 0.0;
        }

        if param_hash == 0x33c22c3c9cu64 {
            //return 0.0;
        }
    }
    original!()(boma, param_type, param_hash)  
}
    

//Hook FS meter timer to activate when LIGHTNING is activated. Change the duration of full meter to essentially infinity, and LIGHTNING to 60 seconds.
#[skyline::hook(offset=0x4e5380)]
pub unsafe fn meter_timer(boma: u64,param_type: u64, param_hash: u64) -> i32 {
    LookupSymbol(&mut FIGHTER_MANAGER_ADDR, "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}".as_bytes().as_ptr());
    //let fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
    //let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);

    if param_type == hash40("common") {

        if param_hash == hash40("charge_final_decrease_start_frame") {

            //return 600; //1 FRAME TO IMMEDIATELY START DECREASING

        }
        if param_hash == hash40("charge_final_remove_frame") {

            //return 600; //10 seconds

        }
    }
    
    original!()(boma, param_type, param_hash)
}
//
//FS METETR
#[fighter_frame_callback]
pub fn fs_meter(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        LookupSymbol(&mut FIGHTER_MANAGER_ADDR, "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}".as_bytes().as_ptr());
        let fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);

        if smash::app::sv_information::is_ready_go() == true {
            //FighterManager::set_final(fighter_manager, smash::app::FighterEntryID(entry_id as i32), smash::app::FighterAvailableFinal { _address: *smash::lib::lua_const::FighterAvailableFinal::CHARGE as u8 }, 0);        
            WorkModule::set_float(fighter.module_accessor, 100.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHARGE_FINAL_ADD_DAMAGE);
        }
        //when meter fills up, remove fs effects on body
        /*if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE) {//FINAL SMASH STATE WILL NEVER ACTIVATE (????), not until button command
            
            WorkModule::unable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
            fighter.clear_lua_stack();
            lua_args!(fighter, 0);                
            sv_animcmd::REMOVE_FINAL_SCREEN_EFFECT(fighter.lua_state_agent);
            fighter.clear_lua_stack();            
            sv_animcmd::FT_REMOVE_FINAL_AURA(fighter.lua_state_agent);    
            if MotionModule::motion_kind(fighter.module_accessor) != hash40("just_shield_off") {
                ModelModule::disable_gold_eye(fighter.module_accessor);	
            } //MISSING HERE, turn off auto fs state
            
            //Different buttons to activate final smash, instead of Neutral B
        }*/

        //Turn meter on in training mode??

        if smashball::is_training_mode() {
            
            //MISSING HERE, turn on meter
        }
    }
}
#[skyline::hook(replace = smash::app::lua_bind::FighterManager::set_final)]
pub unsafe fn final_available_replace(arg1: *mut smash::app::FighterManager, 
    arg2: FighterEntryID, 
    arg3: FighterAvailableFinal, 
    arg4: i32) {

    //let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    //WorkModule::unable_transition_term_forbid(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
    //LIGHTNING_BUTTON[entry_id] = true;
    //return original!()(fighter);
}
//----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


//
pub fn install() {
    smashline::install_agent_frame_callbacks!(fs_meter);
    //skyline::install_hook!(hook_fs_meter);
    //skyline::install_hook!(meter_timer);
    //skyline::install_hook!(final_available_replace);
} 