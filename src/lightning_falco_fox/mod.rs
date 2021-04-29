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
        
        
        if fighter_kind == *FIGHTER_KIND_FALCO {
            if (MotionModule::motion_kind(module_accessor) == smash::hash40("falco_special_n_loop")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("falco_special_air_n_loop"))
            && (ControlModule::get_command_flag_cat(module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 {
                WorkModule::set_flag(module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }

            if MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw")
            ||MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw_r") {
                if MotionModule::frame(module_accessor) >= 7.0 {
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    }
                }
            }
            if MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw_r") {
                if MotionModule::frame(module_accessor) >= 7.0 {
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }
            if MotionModule::motion_kind(module_accessor) == smash::hash40("special_s") {
                if AttackModule:: is_attack_occur(module_accessor) {
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
                    || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }
            
            else if ! (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
            && ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100) {
                if AttackModule:: is_attack_occur(module_accessor) {
                    if MotionModule::frame(module_acessor){ 
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }
        }
        if fighter_kind == *FIGHTER_KIND_FOX {
            if (MotionModule::motion_kind(module_accessor) == smash::hash40("fox_special_n_loop")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("fox_special_air_n_loop"))
            && (ControlModule::get_command_flag_cat(module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 {
                WorkModule::set_flag(module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
             
            if MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw_loop")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw_loop_l")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw_hit")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw_hit_l") {
                if MotionModule::frame(module_accessor) >= 1.0 {
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    }
                }
            }
            if MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw_loop")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw_loop_l")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw_hit_l")
            || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw_hit_l") {
                if MotionModule::frame(module_accessor) >= 1.0 {
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }
                
            else if ! (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
            && ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100) {
                if AttackModule:: is_attack_occur(module_accessor) {
                    if MotionModule::frame(module_acessor){ 
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }
        }
    }                                      
}

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
}