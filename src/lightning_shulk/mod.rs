use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;

// Use this for general per-frame fighter-level hooks
#[fighter_frame( agent = FIGHTER_KIND_SHULK )]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        

        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
            
            if MotionModule::frame(module_accessor) >=30.0 {
                if AttackModule:: is_attack_occur(module_accessor){
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
            
            if MotionModule::frame(module_accessor) >=23.0 {
                if AttackModule:: is_attack_occur(module_accessor){
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
        

        if status_kind == *FIGHTER_STATUS_KIND_THROW {

            if (motion_kind == smash::hash40("throw_f") && frame >=16.0 )
            ||(motion_kind == smash::hash40("throw_b") && frame >=20.0 )
            ||(motion_kind == smash::hash40("throw_hi") && frame >=22.0 )
            ||(motion_kind == smash::hash40("throw_lw") && frame >=25.0 ) 
            {
               if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N != 0 {
                    CancelModule::enable_cancel(module_accessor);
                }
            }  
        }
        
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && MotionModule::frame(module_accessor) >20.0 {
            CancelModule::enable_cancel(module_accessor);
        }
        
        
        else if ! (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
        && ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
            if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(module_accessor){
                CancelModule::enable_cancel(module_accessor);
            }
        }
    }                                      
}

pub fn install() {
    smashline::install_agent_frames!(once_per_fighter_frame);
}