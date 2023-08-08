use super::*;
use crate::fighters::common::mechanics::cancels::attack_cancels::{ENABLE_ATTACK_CANCEL,ENABLE_MULTIHIT_CANCEL,MULTIHIT,MULTIHIT_COUNT};


static mut UP_SPECIAL_HIT : [bool; 8] = [false; 8];
static mut UP_SPECIAL_HIT_COUNT : [i32; 8] = [0; 8];

pub static mut ILLUSION_CANCEL : [bool; 8] = [false; 8];
pub static mut FASTFALL_LASER : [bool; 8] = [false; 8];


#[fighter_frame( agent = FIGHTER_KIND_FALCO )]

    pub fn falco_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            let status_kind = StatusModule::status_kind(module_accessor);
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
            let oboma = sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID)) as u32); // links weapon to whatever may ownn it  
            let o_entry_id = WorkModule::get_int(&mut *oboma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; //links weapon to whatever may own it

            //println!("falco-ill: {}", ILLUSION_CANCEL[entry_id]);

            
            //In Lightning...
            if LIGHTNING[entry_id] {
                //Cancel fair after 3 successful hits    
                let next_input = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 && ! ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_F;
                multihit_counter(fighter, 0, 0, smash::hash40("attack_air_f") , 3, next_input, 0, 0, smash::hash40("attack_air_f"));
            }
            /*
            if motion_kind == hash40("attack_air_f") {
                if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
                    if UP_SPECIAL_HIT[entry_id] == false {
                        UP_SPECIAL_HIT_COUNT[entry_id] +=1;
                        UP_SPECIAL_HIT[entry_id] = true; 
                    }  
                    if UP_SPECIAL_HIT_COUNT[entry_id] >= 3 {
                        UP_SPECIAL_HIT_COUNT[entry_id] = 3;
                        ENABLE_MULTIHIT_CANCEL[entry_id] = true; 
                    }
                    else {
                        ENABLE_MULTIHIT_CANCEL[entry_id] = false;
                    }       
                }
                else {
                    UP_SPECIAL_HIT[entry_id] = false;
                    ENABLE_MULTIHIT_CANCEL[entry_id] = false;
                }  
            }
            else {
                UP_SPECIAL_HIT_COUNT[entry_id] = 0;
            }*/
//Fast fall laser
            if motion_kind == smash::hash40("special_air_n_loop") && FASTFALL_LASER[entry_id] {
                if (ControlModule::get_command_flag_cat(module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 {
                    WorkModule::set_flag(module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                    FASTFALL_LASER[entry_id] = false;
                }                
            }
            else {
                FASTFALL_LASER[entry_id] = false;
            }

//illusion
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
                if AttackModule::is_attack_occur(fighter.module_accessor) {
                    //if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
                    //|| (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0
                    //|| (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {
                        CancelModule::enable_cancel(fighter.module_accessor);
                    //}
                }   
            }
        }
    }

pub fn install() {
    smashline::install_agent_frames!(
        falco_opff,
    );
}