use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::lib::lua_const::*;
use acmd;

// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        //let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);

        //---------------------------------------------------------------------------------------------------------
        //Cancel Up Special
        //-----------------------------------------------------------------------------------------------------------
        //Samus/Dark Samus
        //----------------------    
        //    if fighter_kind == *FIGHTER_KIND_SAMUS || fighter_kind == *FIGHTER_KIND_SAMUSD {
        //        if MotionModule::motion_kind(module_accessor) == smash::hash40("special_hi") {
        //            if MotionModule::frame(module_accessor) == 25.0 {
        //                CancelModule::enable_cancel(module_accessor);
        //            }
        //        }
        //    }
        
        //Zero Suit Samus
        //-----------------------
        
        if fighter_kind == *FIGHTER_KIND_SZEROSUIT {
            if MotionModule::motion_kind(module_accessor) == smash::hash40("special_hi") {
                if MotionModule::frame(module_accessor) == 6.0 || MotionModule::frame(module_accessor) == 10.0 {
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                        //if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0){
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        //}
                    }
                    //if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    //    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
                    //}
                        
                    
                }
                
            }
        }
        if fighter_kind == *FIGHTER_KIND_ROY {
            if MotionModule::motion_kind(module_accessor) == smash::hash40("special_hi") {
                if MotionModule::frame(module_accessor) == 10.0 || MotionModule::frame(module_accessor) == 19.0 {
                    if ! AttackModule::is_attack_occur(module_accessor){
                       if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        } 
                    }  
                }               
            }
        }
        if fighter_kind == *FIGHTER_KIND_CLOUD {
            if MotionModule::motion_kind(module_accessor) == smash::hash40("special_hi") {

                //set clout to wait if up b connects and sheld is pressed (stylistic effect)

                if MotionModule::frame(module_accessor) == 6.0 {
                    if AttackModule::is_attack_occur(module_accessor) {
                        if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0) {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                        }
                    }
                    //if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    //    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
                    //}
                        
                    
                }
                if MotionModule::frame(module_accessor) == 10.0 {
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                        //if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0){
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        //}
                    }
                    //if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    //    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
                    //}
                        
                    
                }
                
            }
        }
           
    }
}

// Use this for general per-frame weapon-level hooks
pub fn once_per_weapon_frame(fighter_base : &mut L2CFighterBase) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
        let frame = smash::app::lua_bind::MotionModule::frame(module_accessor) as i32;

        if frame % 10 == 0 {
            println!("[Weapon Hook] Frame : {}", frame);
        }
    }
}

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
    acmd::add_custom_weapon_hooks!(once_per_weapon_frame);
}