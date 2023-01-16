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


#[fighter_frame( agent = FIGHTER_KIND_EDGE )]
pub fn edge_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        ////let situation_kind = StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        if MotionModule::motion_kind(module_accessor) == hash40("special_hi1_end") && frame >1.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
        if MotionModule::motion_kind(module_accessor) == hash40("special_hi1") {

            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
               ENABLE_ATTACK_CANCEL[entry_id] = true;   
            }
        }
        //Fix Up tilt
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
            if frame >22.0 {
                ENABLE_ATTACK_CANCEL[entry_id] = true;  
            }
            else {//This stays at the bottom
                ENABLE_ATTACK_CANCEL[entry_id] = false;
            } 
        } 
        else {
            ENABLE_ATTACK_CANCEL[entry_id] = true; 
        }
        
    }                                      
}

pub fn install() {
    smashline::install_agent_frames!(edge_opff);

}