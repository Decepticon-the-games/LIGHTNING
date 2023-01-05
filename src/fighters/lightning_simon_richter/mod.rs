use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;
use crate::fighters::common::mechanics::attack_cancels::ENABLE_ATTACK_CANCEL;






#[fighter_frame( agent = FIGHTER_KIND_SIMON )]
pub fn once_per_fighter_frame_simon(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        ////let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            

    
        if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK) 
        && ! (status_kind == *FIGHTER_STATUS_KIND_FINAL)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
        //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3)
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
ENABLE_ATTACK_CANCEL[entry_id] = true;  
}
    }                                      
}

#[fighter_frame( agent = FIGHTER_KIND_RICHTER )]
pub fn once_per_fighter_frame_richter(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        ////let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            

        if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK) 
        && ! (status_kind == *FIGHTER_STATUS_KIND_FINAL)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
        //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3)
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
ENABLE_ATTACK_CANCEL[entry_id] = true;  
}
    }                                      
}

pub fn install() {
    smashline::install_agent_frames!(once_per_fighter_frame_simon, once_per_fighter_frame_richter);
}