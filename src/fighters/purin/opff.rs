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


#[fighter_frame( agent = FIGHTER_KIND_PURIN )]

    pub fn purin_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            let status_kind = StatusModule::status_kind(module_accessor);
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);

            println!("count: {}", DAIR_REST_COUNT[entry_id]);
//Enable cancel  
            static mut UP_SPECIAL_HIT : [bool; 8] = [false; 8];
            static mut UP_SPECIAL_HIT_COUNT : [i32; 8] = [0; 8];
            static mut DAIR_REST_HIT : [bool; 8] = [false; 8];
            static mut DAIR_REST_COUNT : [i32; 8] = [0; 8];

            //Dair
            if motion_kind == hash40("attack_air_lw") {
                //Limit dair to cancel after 3 successful hits
                if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
                    if UP_SPECIAL_HIT[entry_id] == false {
                        UP_SPECIAL_HIT_COUNT[entry_id] +=1;
                        UP_SPECIAL_HIT[entry_id] = true; 
                    }  
                    if UP_SPECIAL_HIT_COUNT[entry_id] >= 3 {
                        UP_SPECIAL_HIT_COUNT[entry_id] = 3;
                        ENABLE_ATTACK_CANCEL[entry_id] = true; 
                    }
                    else {
                        ENABLE_ATTACK_CANCEL[entry_id] = false;
                    }
                    
                    
                }
                else {
                    UP_SPECIAL_HIT[entry_id] = false;
                    ENABLE_ATTACK_CANCEL[entry_id] = false;
                }
                //Dair > rest combo, once before not being able to cancel dair into rest until the next stock 
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                    if DAIR_REST_HIT[entry_id] == false {
                        DAIR_REST_COUNT[entry_id] +=1; 
                        DAIR_REST_HIT[entry_id] = true;
                    }
                    if DAIR_REST_COUNT[entry_id] >= 1 {
                        DAIR_REST_COUNT[entry_id] = 1;
                        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW)
                    }
                }  
            }
            else {
                ENABLE_ATTACK_CANCEL[entry_id] = true;
            }

            //RESETS
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
                DAIR_REST_COUNT[entry_id] = 0;
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW)
                
            } 
            
        


//New subtititle for any other code, if not applicable just delete the lines

        }
    }

pub fn install() {
    smashline::install_agent_frames!(purin_opff);

}