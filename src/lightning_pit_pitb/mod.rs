use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;

// Use this for general per-frame fighter-level hooks
#[fighter_frame( agent = FIGHTER_KIND_PIT )]
pub fn once_per_fighter_frame_pit(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        //let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4{
            if MotionModule::frame(module_accessor) >10.0 {
                    if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(module_accessor){
                CancelModule::enable_cancel(module_accessor);
            }}
        }
            
            
    
        else if AttackModule:: is_attack_occur(fighter.module_accessor) {
            CancelModule::enable_cancel(module_accessor);
            
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_PITB )]
pub fn once_per_fighter_frame_pitb(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        //let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4{
            if MotionModule::frame(module_accessor) >10.0 {
                    if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(module_accessor){
                CancelModule::enable_cancel(module_accessor);
            }}
        }
            
            
    
        else if AttackModule:: is_attack_occur(fighter.module_accessor) {
            CancelModule::enable_cancel(module_accessor);
            
        }
    }                                      
}

pub fn install() {
    smashline::install_agent_frames!(once_per_fighter_frame_pit, once_per_fighter_frame_pitb);
}