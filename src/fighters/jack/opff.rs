use {
    smash::{
        lua2cpp::{L2CAgentBase,L2CFighterCommon},
        phx::Hash40,
        hash40,
        app::{lua_bind::*, sv_animcmd::*,*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};
use crate::fighters::common::mechanics::attack_cancels::ENABLE_ATTACK_CANCEL;






#[fighter_frame( agent = FIGHTER_KIND_JACK )]
pub fn jack_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(module_accessor);
        //let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 1);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        //let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        let frame = MotionModule::frame(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor); 
        

        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
            if frame >=20.0 {
                ENABLE_ATTACK_CANCEL[entry_id] = true;
            }
            else {
                ENABLE_ATTACK_CANCEL[entry_id] = false;
            }
            
        } 
        else if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3 {
            if frame >=12.0 {
                ENABLE_ATTACK_CANCEL[entry_id] = true;
            }
            else {
                ENABLE_ATTACK_CANCEL[entry_id] = false;
            }
            
        } 
        else if motion_kind == smash::hash40("attack_air_hi") {
                
            if frame >= 19.0 {
                ENABLE_ATTACK_CANCEL[entry_id] = true;
            } 
            else {
                ENABLE_ATTACK_CANCEL[entry_id] = false;
            } 
        }
        else {
            ENABLE_ATTACK_CANCEL[entry_id] = true; 
        }
    }                                      
}

pub fn install() {
    smashline::install_agent_frames!(jack_opff);
}