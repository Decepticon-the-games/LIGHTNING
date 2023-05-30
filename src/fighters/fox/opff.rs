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
use crate::fighters::common::mechanics::attack_cancels::{ENABLE_ATTACK_CANCEL,ENABLE_MULTIHIT_CANCEL};
use crate::fighters::common::mechanics::lightning_mode::LIGHTNING;





#[fighter_frame( agent = FIGHTER_KIND_FOX )]

    pub fn fox_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            let status_kind = StatusModule::status_kind(module_accessor);
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
            let max_jumps = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
            let jumps_used = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);


//Fast fall laser
            if (motion_kind == smash::hash40("special_n_loop")
            || motion_kind == smash::hash40("special_air_n_loop"))
            && (ControlModule::get_command_flag_cat(module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 {
                WorkModule::set_flag(module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }

//jump cancel shine (multishine)
            if  (status_kind == *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP && frame >1.0 ) {
                if (max_jumps == 2 && jumps_used <2)
                {
                    if (ControlModule::is_enable_flick_jump(fighter.module_accessor) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
                        if situation_kind == *SITUATION_KIND_AIR {
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                        }
                        if situation_kind == *SITUATION_KIND_GROUND {
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false);
                        }
                        else {
                            CancelModule::enable_cancel(fighter.module_accessor);
                        }
                    }
                }
            }
////multihit cancels

            static mut MULTIHIT : [bool; 8] = [false; 8];
            static mut MULTIHIT_COUNT : [i32; 8] = [0; 8];


            if ENABLE_MULTIHIT_CANCEL[entry_id] && LIGHTNING[entry_id] {
                if motion_kind == hash40("attack_air_lw")  {
                    ENABLE_ATTACK_CANCEL[entry_id] = false;
                    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
                        if MULTIHIT[entry_id] == false {
                            MULTIHIT_COUNT[entry_id] +=1;
                            MULTIHIT[entry_id] = true; 
                        }         
                    }
                    else {
                        MULTIHIT[entry_id] = false;
                    }  

                    if MULTIHIT_COUNT[entry_id] >= 3 { //how many hits
                        MULTIHIT_COUNT[entry_id] = 3;  //how many hits
                        ENABLE_ATTACK_CANCEL[entry_id] = true; 
                    }
                    else {
                        ENABLE_ATTACK_CANCEL[entry_id] = false;
                    } 
                }
                else {
                    //ENABLE_ATTACK_CANCEL[entry_id] = true; 
                    MULTIHIT_COUNT[entry_id] = 0;
                }     
            }            
        }
    }


   
pub fn install() {
    smashline::install_agent_frames!(fox_opff);

}