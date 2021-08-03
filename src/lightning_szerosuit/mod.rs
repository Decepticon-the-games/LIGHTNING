use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;

// Use this for general per-frame fighter-level hooks
#[fighter_frame( agent = FIGHTER_KIND_SZEROSUIT )]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        //FIX  
        
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4
        || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {

            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
        
                if MotionModule::frame(module_accessor) >=28.0 {
                    if AttackModule:: is_attack_occur(module_accessor){
                        CancelModule::enable_cancel(module_accessor);
                    }
                } 
                                
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
        
                if MotionModule::frame(module_accessor) >=26.0 {
                    if AttackModule:: is_attack_occur(module_accessor){
                        CancelModule::enable_cancel(module_accessor);
                    }
                } 
                                
            }
            
        }
        else if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3) {
            if AttackModule:: is_attack_occur(module_accessor)  &&  ! AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT) {
                CancelModule::enable_cancel(module_accessor);
            }
        }

        //ENHANCE 

        if MotionModule::frame(module_accessor) == 23.0 || MotionModule::frame(module_accessor) == 26.0 {
            if AttackModule::is_attack_occur(module_accessor) {
                if cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
                }
            }
        }
    }                                      
}

pub fn install() {
    smashline::install_agent_frames!(once_per_fighter_frame);
}