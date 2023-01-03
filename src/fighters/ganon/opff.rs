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


#[fighter_frame( agent = FIGHTER_KIND_GANON )]

    pub fn ganon_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
            //let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);

//Enable cancel   

            //Up special
            if status_kind == *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING {
                ENABLE_ATTACK_CANCEL[entry_id] = false;
            }
            //up tilt 
            else if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
                ENABLE_ATTACK_CANCEL[entry_id] = false;
            }
            else {
                ENABLE_ATTACK_CANCEL[entry_id] = true;
            }


//Cancel up tilt to the attack once a hitbox connects
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 && (frame > 23.0 && frame < 53.0) {
                if AttackModule::is_attack_occur(module_accessor) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
            
                    MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 53.0, false, false, false);
                    
                }
                if frame > 64.0 {
                    if ! (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
                        ENABLE_ATTACK_CANCEL[entry_id] = true; 
                    }  
                }
            }
//Taunt/Attack cancel side special
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
                if frame>= 18.0 {
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
                    || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)
                    || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
                    || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
                    || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                        crate::fighters::common::utility::enable_cancel_real(fighter);
                    }
                }
            }

        }
    }

pub fn install() {
    smashline::install_agent_frames!(ganon_opff);

}