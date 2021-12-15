
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::lib::lua_const::*;
use smashline::*;

// Use this for general per-frame fighter-level hooks
#[fighter_frame( agent = FIGHTER_KIND_MIIFIGHTER )]
fn once_per_fighter_frame_1(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
        let jump_guard_dash_upspecial_pressed = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0 || (situation_kind == *SITUATION_KIND_GROUND && (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0) || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0);
        let frame = MotionModule::frame(module_accessor);
        
        //FIXES
        //-------------------------------------------------------------------------------
        if (status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S1_END) && frame >= 25.0
        {
            if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(module_accessor){
                if  jump_guard_dash_upspecial_pressed {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }

        else if (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
        && ! (status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_HIT)
        && ! (status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S1_END)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
            if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(module_accessor){
                CancelModule::enable_cancel(module_accessor);
            }
        }
    }                                      
}

#[fighter_frame( agent = FIGHTER_KIND_MIIGUNNER )]
fn once_per_fighter_frame_2(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
        let jump_guard_dash_upspecial_pressed = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0 || (situation_kind == *SITUATION_KIND_GROUND && (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0) || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0);
        let frame = MotionModule::frame(module_accessor);
        
        //FIXES
        //-------------------------------------------------------------------------------
        
        if (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
            if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(module_accessor){
                CancelModule::enable_cancel(module_accessor);
            }
        }
    }                                      
}

#[fighter_frame( agent = FIGHTER_KIND_MIISWORDSMAN )]
fn once_per_fighter_frame_3(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
        let jump_guard_dash_upspecial_pressed = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0 || (situation_kind == *SITUATION_KIND_GROUND && (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0) || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0);
        let frame = MotionModule::frame(module_accessor);
        
        //FIXES
        //-------------------------------------------------------------------------------
        if (status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_END) && frame >32.0  
        || (status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_END_MAX) && frame >32.0  
        {
            if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(module_accessor){
                if  jump_guard_dash_upspecial_pressed {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
        else if (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
        && ! (status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_END)
        && ! (status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N3_END_MAX)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
            if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(module_accessor){
                CancelModule::enable_cancel(module_accessor);
            }
        }
    }                                      
}

pub fn install() {
    smashline::install_agent_frames!(once_per_fighter_frame_1,once_per_fighter_frame_2,once_per_fighter_frame_3);
}