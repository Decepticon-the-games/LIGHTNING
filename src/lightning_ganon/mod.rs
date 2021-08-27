use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;

// Use this for general per-frame fighter-level hooks
#[fighter_frame( agent = FIGHTER_KIND_GANON )]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        //let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        //if MotionModule::motion_kind(module_accessor)== smash::hash40("throw_lw"){
        //    if MotionModule::frame(module_accessor)>= 25.0 {
        //        CancelModule::enable_cancel(module_accessor);
        //    }

        //}
        //if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
        //    if MotionModule::frame(module_accessor)>= 64.0 {
                //if AttackModule::is_attack_occur(module_accessor){
        //            CancelModule::enable_cancel(module_accessor);
                //}
        //    }

        //}
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            if MotionModule::frame(module_accessor)>= 18.0 {
                if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
                || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)
                || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
                || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
                || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
            

        }    
        if MotionModule::motion_kind(module_accessor)== smash::hash40("special_s") {
            if MotionModule::frame(module_accessor)>= 40.0 {
                CancelModule::enable_cancel(module_accessor);   
            }               
        }        
        if MotionModule::motion_kind(module_accessor)== smash::hash40("special_air_s") {
            if MotionModule::frame(module_accessor)>= 3.0 {
                CancelModule::enable_cancel(module_accessor);   
            }               
        }     
        
        else if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
        && ! (status_kind == *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3)
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
                if AttackModule:: is_attack_occur(fighter.module_accessor) {

                    if ! AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT) { 

                        CancelModule::enable_cancel(module_accessor);
                }
            }
        }
    }                                      
}

pub fn install() {
    smashline::install_agent_frames!(once_per_fighter_frame);
}