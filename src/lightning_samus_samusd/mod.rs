use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
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
        
        
        if fighter_kind == *FIGHTER_KIND_SAMUS || fighter_kind == *FIGHTER_KIND_SAMUSD
        && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
                
                if MotionModule::frame(module_accessor) == 15.0 {
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                        //StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
                    }
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