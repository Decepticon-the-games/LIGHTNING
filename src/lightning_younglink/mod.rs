
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon};
use smash::lib::lua_const::*;
use smashline::*;

// Use this for general per-frame fighter-level hooks
#[fighter_frame( agent = FIGHTER_KIND_YOUNGLINK )]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let frame = MotionModule::frame(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);  
        let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 1);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        let jump_guard_dash_upspecial_pressed = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0 || (situation_kind == *SITUATION_KIND_GROUND && (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0) || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0);
    
        //FIXES
        //-------------------------------------------------------------------------------
        
 
        //Up Smash
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 && MotionModule::frame(module_accessor) >39.0 {            
            if AttackModule::is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(module_accessor){
                CancelModule::enable_cancel(module_accessor);
            }
        }

        //Up Special
        if (motion_kind == smash::hash40("special_hi") && frame >= 43.0) || (motion_kind == smash::hash40("special_air_hi") && frame >= 46.0)  {
            if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(module_accessor) {
                if jump_guard_dash_upspecial_pressed {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
        
    
        //else 
        else if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (motion_kind == smash::hash40("special_hi") || motion_kind == smash::hash40("special_air_hi"))
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4)
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
            if AttackModule::is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(module_accessor) {
                CancelModule::enable_cancel(module_accessor);
            }
        }

        //ENHANCES
        //--------------------------------------------------------------------------------


        //MOTION CANCELS
        //--------------------------------------------------------------------------------
    }                                      
}

pub fn install() {
    smashline::install_agent_frames!(once_per_fighter_frame);
}