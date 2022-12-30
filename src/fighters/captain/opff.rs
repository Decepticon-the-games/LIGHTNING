use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;
use crate::lightning_01_common::ATTACK_CANCEL;






#[fighter_frame( agent = FIGHTER_KIND_CAPTAIN )]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        //let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
        let frame = MotionModule::frame(fighter.module_accessor);
        ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        ////let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        
        //Fix Up Smash 
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 && frame >27.0 {
                     if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor) {
                        CancelModule::enable_cancel(fighter.module_accessor);
                        ATTACK_CANCEL[entry_id] = true; 
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
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
            }
            //CANCEL ON HIT WITH GRAB AND A ATTACKS
            if AttackModule:: is_attack_occur(module_accessor){
                if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CATCH){
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
                if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
            }

        }    
            
        if ! (status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_HI_CLING)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
                     if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor) {
                        CancelModule::enable_cancel(fighter.module_accessor);
                        ATTACK_CANCEL[entry_id] = true; 
                    }   
}
        
    }                                      
}

pub fn install() {
    smashline::install_agent_frames!(once_per_fighter_frame);
}