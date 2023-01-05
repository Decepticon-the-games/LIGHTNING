
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon};
use smash::lib::lua_const::*;
use smashline::*;
use crate::fighters::common::mechanics::attack_cancels::ENABLE_ATTACK_CANCEL;






#[fighter_frame( agent = FIGHTER_KIND_ELIGHT )]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        ////let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        //Fix Neutral B/Side B
        if MotionModule::motion_kind(module_accessor) == smash::hash40("special_n2") 
        || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_n2"){
            if MotionModule::frame(module_accessor) >44.0 {
                ENABLE_ATTACK_CANCEL[entry_id] = true; 
            }
            else {
                ENABLE_ATTACK_CANCEL[entry_id] = false;
            }
        }

        if MotionModule::motion_kind(module_accessor) == smash::hash40("special_n")
        || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_n") {
            if MotionModule::frame(module_accessor) >35.0 {
                ENABLE_ATTACK_CANCEL[entry_id] = true; 
            }
            else {
                ENABLE_ATTACK_CANCEL[entry_id] = false;
            }
        }
        
         
        if MotionModule::motion_kind(module_accessor) == smash::hash40("special_s") 
        || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_s") {
            if MotionModule::frame(module_accessor) >14.0 {
                ENABLE_ATTACK_CANCEL[entry_id] = true; 
            }
            else {
                ENABLE_ATTACK_CANCEL[entry_id] = false;
            }
        }
        else if status_kind == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_N_END
        || status_kind == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD {
            ENABLE_ATTACK_CANCEL[entry_id] = false;
        }
        else {
            ENABLE_ATTACK_CANCEL[entry_id] = true; 
        }
    }                                      
}

pub fn install() {
    smashline::install_agent_frames!(once_per_fighter_frame);
}