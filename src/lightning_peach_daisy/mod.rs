
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon};
use smash::lib::lua_const::*;


// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        //let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        //let jump_guard_dash_upspecial_pressed = ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) || (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0;
        //
        
        
        if fighter_kind == *FIGHTER_KIND_PEACH || fighter_kind == *FIGHTER_KIND_DAISY {

            //FIXES
            //-------------------------------------------------------------------------------
           
                
                
        
            //else 
            if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100) {
                if AttackModule:: is_attack_occur(module_accessor) {
                    if MotionModule::frame(module_accessor) >1.0 { 
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }

            //ENHANCES
            //--------------------------------------------------------------------------------


            //MOTION CANCELS
            //--------------------------------------------------------------------------------

        }
        
    }                                      
}

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
}