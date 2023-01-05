
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon};
use smash::lib::lua_const::*;
use smashline::*;
use crate::fighters::common::mechanics::attack_cancels::ENABLE_ATTACK_CANCEL;






#[fighter_frame( agent = FIGHTER_KIND_PALUTENA )]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 1);
        ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        ////let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
            if MotionModule::frame(module_accessor) >23.0 {
                if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(module_accessor){
                    if prev_status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
                        CancelModule::enable_cancel(fighter.module_accessor);
                    }
                }               
            }
        }
            
            
    
        else {
            ENABLE_ATTACK_CANCEL[entry_id] = true;  
        }    
    }                                      
}

pub fn install() {
    smashline::install_agent_frames!(once_per_fighter_frame);
}