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





#[fighter_frame( agent = FIGHTER_KIND_PIKACHU )]

    pub fn pikachu_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            //let status_kind = StatusModule::status_kind(module_accessor);
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            //let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);

//Enable cancel subtitle here.   

             
            if ENABLE_MULTIHIT_CANCEL[entry_id] && LIGHTNING[entry_id] {
                
                static mut MULTIHIT : [bool; 8] = [false; 8];
                static mut MULTIHIT_COUNT : [i32; 8] = [0; 8];

                if motion_kind == smash::hash40("attack_air_n") 
                || motion_kind == smash::hash40("attack_air_f") 
                || motion_kind == smash::hash40("attack_air_b") {
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
    smashline::install_agent_frames!(pikachu_opff);

}