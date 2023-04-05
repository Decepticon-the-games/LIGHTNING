use {
    smash::{
        lua2cpp::{L2CAgentBase,L2CFighterCommon},
        phx::Hash40,
        hash40,
        app::{lua_bind::*, sv_animcmd::*,*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};
use crate::fighters::common::mechanics::attack_cancels::ENABLE_ATTACK_CANCEL;


#[fighter_frame( agent = FIGHTER_KIND_SZEROSUIT )]

    pub fn szerosuit_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            let status_kind = StatusModule::status_kind(module_accessor);
            //let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);

//Enable cancel  
//println!("enable: {}", ENABLE_ATTACK_CANCEL[entry_id]);

            
            //else if for every new move.  
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
                
                if frame >= 28.0 {
                    ENABLE_ATTACK_CANCEL[entry_id] = true;
                }  
                else {
                    ENABLE_ATTACK_CANCEL[entry_id] = false;
                }
            }
            else if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
                
                if frame >= 9.0 {
                    ENABLE_ATTACK_CANCEL[entry_id] = true;
                }  
                else {
                    ENABLE_ATTACK_CANCEL[entry_id] = false;
                }
            }
            else if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
                
                if frame >= 26.0 {
                    ENABLE_ATTACK_CANCEL[entry_id] = true;
                }  
                else {
                    ENABLE_ATTACK_CANCEL[entry_id] = false;
                }
            }
            else if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                ENABLE_ATTACK_CANCEL[entry_id] = false;
            }
            else {//This stays at the bottom
                ENABLE_ATTACK_CANCEL[entry_id] = true;
            }


//Cancel up special into down special before the last hit if sucesfully hit

            if MotionModule::frame(module_accessor) == 23.0 || MotionModule::frame(module_accessor) == 26.0 {
                if AttackModule::is_attack_occur(module_accessor) && ! SlowModule::is_slow(module_accessor) {
                    if cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
                    }
                }
            }
        }
    }

pub fn install() {
    smashline::install_agent_frames!(szerosuit_opff);

}