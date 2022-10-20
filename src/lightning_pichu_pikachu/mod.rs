
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon};
use smash::lib::lua_const::*;
use smashline::*;
use crate::lightning_01_common::ATTACK_CANCEL;






#[fighter_frame( agent = FIGHTER_KIND_PICHU )]
pub fn once_per_fighter_frame_pichu(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);  
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
        let jump_guard_dash_upspecial_pressed = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0 || (situation_kind == *SITUATION_KIND_GROUND && (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0) || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0);
    
        //FIXES
        //-------------------------------------------------------------------------------
        //Fix FAIR chain
        if motion_kind == smash::hash40("attack_air_f") && frame >= 17.0 
        || motion_kind == smash::hash40("attack_air_b") && frame >= 25.0
        {
 
                if  jump_guard_dash_upspecial_pressed {
                    ATTACK_CANCEL[entry_id] = true; 
                }

        }
        
            
            
    
        //else 
        else if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK) 
        && ! (status_kind == *FIGHTER_STATUS_KIND_FINAL)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        && !  motion_kind == smash::hash40("attack_air_f")
        && ! motion_kind == smash::hash40("attack_air_b")
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
ATTACK_CANCEL[entry_id] = true;  
}

        //ENHANCES
        //--------------------------------------------------------------------------------


        //MOTION CANCELS
        //--------------------------------------------------------------------------------
    }                                      
}

#[fighter_frame( agent = FIGHTER_KIND_PIKACHU )]
pub fn once_per_fighter_frame_pikachu(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor); 
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
        let jump_guard_dash_upspecial_pressed = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0 || (situation_kind == *SITUATION_KIND_GROUND && (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0) || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0);
    
        //FIXES
        //-------------------------------------------------------------------------------
        //Fix FAIR chain
        if motion_kind == smash::hash40("attack_air_f") && frame >= 26.0 
        || motion_kind == smash::hash40("attack_air_b") && frame >= 19.0
        || motion_kind == smash::hash40("attack_air_n") && frame >= 19.0
        {

                if  jump_guard_dash_upspecial_pressed {
                    ATTACK_CANCEL[entry_id] = true; 
                }
        
        }
        
            
            
    
        //else 
        else if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK) 
        && ! (status_kind == *FIGHTER_STATUS_KIND_FINAL)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        && !  motion_kind == smash::hash40("attack_air_f")
        && ! motion_kind == smash::hash40("attack_air_b")
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
ATTACK_CANCEL[entry_id] = true;  
}

        //ENHANCES
        //--------------------------------------------------------------------------------


        //MOTION CANCELS
        //--------------------------------------------------------------------------------
    }                                      
}
pub fn install() {
    smashline::install_agent_frames!(once_per_fighter_frame_pichu, once_per_fighter_frame_pikachu);
}