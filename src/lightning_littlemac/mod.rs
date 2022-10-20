use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;
use crate::lightning_01_common::ATTACK_CANCEL;






#[fighter_frame( agent = FIGHTER_KIND_LITTLEMAC )]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        //let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {

            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
                if MotionModule::frame(module_accessor) >=7.0 {
ATTACK_CANCEL[entry_id] = true; 
                }
            }
        } 
            
        else if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK) 
        && ! (status_kind == *FIGHTER_STATUS_KIND_FINAL)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
        //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3)
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
ATTACK_CANCEL[entry_id] = true;  
}
        //NO SMASH ATTACK TO JAB
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4 {
            if AttackModule:: is_attack_occur(fighter.module_accessor) {
                //CancelModule::enable_cancel(fighter.module_accessor);    
            }
        }
    }                                      
}

pub fn install() {
    smashline::install_agent_frames!(once_per_fighter_frame);
}