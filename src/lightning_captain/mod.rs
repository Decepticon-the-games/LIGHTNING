use smash::app::lua_bind::StatusModule::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::lib::lua_const::*;
use acmd;

// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        
        if fighter_kind == *FIGHTER_KIND_CAPTAIN {
            //FIX DOWN TRHROW, IT HAS NO HITBOX 
            if MotionModule::motion_kind(module_accessor)== smash::hash40("throw_lw"){ 
                if MotionModule::frame(module_accessor)>= 22.0 {
                    CancelModule::enable_cancel(module_accessor);
                }

            }
            //SIDE B
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
                //TAUNT CANCEL
                if MotionModule::frame(module_accessor)>= 20.0 {
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
                    || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)
                    || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
                    || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
                //CANCEL ON HIT WITH GRAB AND A ATTACKS
                if AttackModule:: is_attack_occur(module_accessor){
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CATCH){
                        CancelModule::enable_cancel(module_accessor);
                    }
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                        CancelModule::enable_cancel(module_accessor);
                    }
                }

            }    
                
            else if ! (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
            && ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100) {
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