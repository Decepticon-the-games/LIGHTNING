
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::lib::lua_const::*;
use smashline::*;
use crate::lightning_01_ultrainstinct::FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CROSS_CANCEL;


pub static mut SPECIAL_MECHANICS_METER_ENABLE_COUNT : [bool; 8] = [true; 8]; //one frame
pub static mut SPECIAL_MECHANICS_METER_COUNT : [f32; 8] = [0.0; 8];
static mut DAMAGE_GIVEN : [f32; 8] = [0.0; 8];
static mut DAMAGE_TAKEN : [f32; 8] = [0.0; 8];
static mut INIT_DAMAGE : [f32; 8] = [0.0; 8];


#[fighter_frame_callback]
fn cross_meter(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(module_accessor);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        //let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 1);
        //let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
        //let jump_guard_dash_upspecial_pressed = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0);
        let frame = MotionModule::frame(module_accessor);
        
//SPECIAL MECHANICS METER:: 100 pts, every special mechanic will cost meter. No meter, no mechanic

        //println!("meter: {}", SPECIAL_MECHANICS_METER_COUNT[entry_id]);
        //println!("stick x: {}", );


        if SPECIAL_MECHANICS_METER_COUNT[entry_id] >= 0.0 {

            if StopModule::is_damage(module_accessor) {

                //rack up by damage given by direct attack, no projectiless cuz we aint no campers



                //rack up by damage taken
                if SPECIAL_MECHANICS_METER_ENABLE_COUNT[entry_id] {
                    
                    SPECIAL_MECHANICS_METER_COUNT[entry_id] += (DamageModule::damage(module_accessor, 0) * 0.09);
                    SPECIAL_MECHANICS_METER_ENABLE_COUNT[entry_id] = false;
                }
            }        
            else {
                SPECIAL_MECHANICS_METER_ENABLE_COUNT[entry_id] = true;
            } 

            if SPECIAL_MECHANICS_METER_COUNT[entry_id] >= 100.0 { //Cap at 100 pts
                SPECIAL_MECHANICS_METER_COUNT[entry_id] = 100.0;
            }

            //WHEN buttons are pressed, subtract meter

            if FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CROSS_CANCEL[entry_id] {
                SPECIAL_MECHANICS_METER_COUNT[entry_id] -= 10.0; //TAKE 10 POINTS AWAY
            }
        }
        else {
            SPECIAL_MECHANICS_METER_COUNT[entry_id] = 0.0;
        }    
        
        //RESET
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
                    
            SPECIAL_MECHANICS_METER_COUNT[entry_id] = 0.0;
            SPECIAL_MECHANICS_METER_ENABLE_COUNT[entry_id] = true;
        }

        
    }                                      
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(cross_meter);
}