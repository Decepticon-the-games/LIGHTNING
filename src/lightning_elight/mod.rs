
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon};
use smash::lib::lua_const::*;
use acmd;

// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        //let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        
        if fighter_kind == *FIGHTER_KIND_ELIGHT {
            //Fix Neutral B
            if (MotionModule::motion_kind(module_accessor) == smash::hash40("special_n") && MotionModule::frame(module_accessor) >=29.0) 
            || (MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_n") && MotionModule::frame(module_accessor) >=29.0) 
            || (MotionModule::motion_kind(module_accessor) == smash::hash40("special_n2") && MotionModule::frame(module_accessor) >=38.0)
            || (MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_n2") && MotionModule::frame(module_accessor) >38.0) {
                if AttackModule:: is_attack_occur(module_accessor) {
                        CancelModule::enable_cancel(module_accessor);
                }
            }
                
                
        
            else if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3) {
                if AttackModule:: is_attack_occur(module_accessor) {
                        CancelModule::enable_cancel(module_accessor);
                }
            }
        }
    }                                      
}

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
}