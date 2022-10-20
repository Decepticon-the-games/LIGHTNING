use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon};
use smash::lib::lua_const::*;
use smashline::*;
use crate::lightning_01_common::ATTACK_CANCEL;






//#[fighter_frame( agent = FIGHTER_KIND_TRAIL )]
#[fighter_frame_callback]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let motion_kind = MotionModule::motion_kind(module_accessor);       
        //let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
        //let jump_guard_dash_upspecial_pressed = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0 || (situation_kind == *SITUATION_KIND_GROUND && (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0) || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0);
    
        if fighter_kind == *FIGHTER_KIND_DEMON+1 {
        //FIXES
        //-------------------------------------------------------------------------------
        //Fix 
        if motion_kind== smash::hash40("attack_s3_3") || motion_kind== smash::hash40("attack_air_n3") || motion_kind== smash::hash40("attack_air_f3") {
ATTACK_CANCEL[entry_id] = true; 
         
        }     
        if motion_kind == smash::hash40("attack_hi3") && frame >=32.0 {
ATTACK_CANCEL[entry_id] = true; 
         
        }
    
        //else 
        if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK) 
        && ! (status_kind == *FIGHTER_STATUS_KIND_FINAL)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        && ! (motion_kind== smash::hash40("attack_s3"))
        && ! (motion_kind== smash::hash40("attack_s3_2"))
        //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3)
        && ! (motion_kind== smash::hash40("attack_air_n"))
        && ! (motion_kind== smash::hash40("attack_air_n2"))
        && ! (motion_kind== smash::hash40("attack_air_f"))
        && ! (motion_kind== smash::hash40("attack_air_f2"))
        //&& ! (status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N)
        //&& ! (status_kind == FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F)
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
ATTACK_CANCEL[entry_id] = true;  
}

        //ENHANCES
        //--------------------------------------------------------------------------------


        //MOTION CANCELS
        //--------------------------------------------------------------------------------
        }
    }                                      
}

pub fn install() {
    //smashline::install_agent_frames!(once_per_fighter_frame);
    smashline::install_agent_frame_callbacks!(once_per_fighter_frame);
}