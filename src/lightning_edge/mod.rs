use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;



use smash::hash40;

// Use this for general per-frame fighter-level hooks
#[fighter_frame( agent = FIGHTER_KIND_EDGE )]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        //let situation_kind = StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        if MotionModule::motion_kind(module_accessor) == hash40("special_hi1_end") && frame >1.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
        if MotionModule::motion_kind(module_accessor) == hash40("special_hi1") {
            if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(module_accessor){
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
            }
        }
        //Fix Up tilt
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 && frame >22.0 {
                        if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor) && ! status_kind == *FIGHTER_STATUS_KIND_FINAL {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        
        }
            
        if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3)
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
                        if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor) && ! status_kind == *FIGHTER_STATUS_KIND_FINAL {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        
        }
        
    }                                      
}

pub fn install() {
    smashline::install_agent_frames!(once_per_fighter_frame);
}