use super::*;
use crate::fighters::common::mechanics::cancels::attack_cancels::{ENABLE_ATTACK_CANCEL,ENABLE_MULTIHIT_CANCEL};
pub static mut DAIR_REST_NOKILL : [bool; 8] = [false; 8];

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

            
            static mut MULTIHIT : [bool; 8] = [false; 8];
            static mut MULTIHIT_COUNT : [i32; 8] = [0; 8];


            //Dair

            //In Lightning...
            if LIGHTNING[entry_id] {
                //Dair cancels after 3 successful hits into fair, upair, specials 
                let next_input = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0;
                multihit_counter(fighter, 0, 0, smash::hash40("attack_air_lw"), 3, next_input, 0, 0, smash::hash40("attack_air_lw"));
            }

        
            /*if motion_kind == hash40("attack_air_lw") {
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
                    ENABLE_MULTIHIT_CANCEL[entry_id] = true; 
                }
                else {
                    ENABLE_MULTIHIT_CANCEL[entry_id] = false;
                } 
                
            }
            else {
                ////ENABLE_ATTACK_CANCEL[entry_id] = true; 
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_CATEGORY_MASK_ALL) {
            ENABLE_ATTACK_CANCEL[entry_id] = true; 
        }
                MULTIHIT_COUNT[entry_id] = 0;
            }*/


            //Dair > rest combo nerf
            if motion_kind == smash::hash40("attack_air_lw") 
            && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                //if DAIR_REST_HIT[entry_id] == false {
                //    DAIR_REST_COUNT[entry_id] +=1; 
                //    DAIR_REST_HIT[entry_id] = true;
                //}
                //if DAIR_REST_COUNT[entry_id] >= 1 {
                //    DAIR_REST_COUNT[entry_id] = 1;
                    DAIR_REST_NOKILL[entry_id] = true;
                //}
            }  
            

            //RESETS
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
                //DAIR_REST_COUNT[entry_id] = 0;
                DAIR_REST_NOKILL[entry_id] = false;
            } 
            
        


//New subtititle for any other code, if not applicable just delete the lines

        }
    }

pub fn install() {
    smashline::install_agent_frames!(purin_opff);

}