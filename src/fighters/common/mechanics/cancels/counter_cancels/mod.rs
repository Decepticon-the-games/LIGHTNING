use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;
use crate::fighters::common::mechanics::cancels::attack_cancels::ENABLE_ATTACK_CANCEL;
use crate::fighters::common::mechanics::cancels::motioncancels::CANCEL_IN_NEUTRAL;

pub static mut COUNTER_CANCEL : [bool; 8] = [false; 8];

#[fighter_frame_callback]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let fighter_kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
        //let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

        //println!("cc: {}", COUNTER_CANCEL[entry_id]);
        if CANCEL_IN_NEUTRAL[entry_id] || ENABLE_ATTACK_CANCEL[entry_id] {//TRY IS_ATTACK_OCCUR AND ELSE INPUT
            COUNTER_CANCEL[entry_id] = false;
        }

        if COUNTER_CANCEL[entry_id] {
            if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                CancelModule::enable_cancel(fighter.module_accessor);
                COUNTER_CANCEL[entry_id] = false;
            }
            else if fighter_kind == *FIGHTER_KIND_SHULK && ! ((cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) || (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0)) {
                CancelModule::enable_cancel(fighter.module_accessor);
                COUNTER_CANCEL[entry_id] = false;
            }
        }        
        //MARTH, LUCINA, ROY, CHROM, SEPHIROTH, CORRIN
        if /*fighter_kind == *FIGHTER_KIND_MARTH 
        || fighter_kind == *FIGHTER_KIND_LUCINA
        || fighter_kind == *FIGHTER_KIND_ROY
        || fighter_kind == *FIGHTER_KIND_CHROM
        || fighter_kind == *FIGHTER_KIND_IKE
        || fighter_kind == *FIGHTER_KIND_KAMUI
        || */fighter_kind == *FIGHTER_KIND_EDGE {

            if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_lw_hit") || MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_air_lw_hit") {
                if MotionModule::frame(fighter.module_accessor) >1.0 {
                    if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                        CancelModule::enable_cancel(fighter.module_accessor);
                    }
                }
            }   
        }
        //SHULK
        /*if fighter_kind == *FIGHTER_KIND_SHULK {
            if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_lw_hit") || MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_air_lw_hit") {
                if ! ((cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) || (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0)) {
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
            }
        }*/
        //KROOL
        if fighter_kind == *FIGHTER_KIND_KROOL {
            if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_lw_hit") || MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_air_lw_hit") {
                if MotionModule::frame(fighter.module_accessor) >2.0 {
                    if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                        CancelModule::enable_cancel(fighter.module_accessor);
                    }
                }
            }   
        }
        //LUCARIO
        /*if fighter_kind == *FIGHTER_KIND_LUCARIO {
            if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_lw_split") {
                if MotionModule::frame(fighter.module_accessor) >1.0 {
                    if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                        CancelModule::enable_cancel(fighter.module_accessor);
                    }
                }
            }   
        }*/
        //GRENINJA
        /*if fighter_kind == *FIGHTER_KIND_GEKKOUGA {
            if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_lw_hit") || MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_air_lw_hit") {
                if MotionModule::frame(fighter.module_accessor) >1.0 {
                    if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                        CancelModule::enable_cancel(fighter.module_accessor);
                    }
                }
            }   
        }*/
        //JOKER
        if fighter_kind == *FIGHTER_KIND_JACK {
            if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_lw_counter") || MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_air_lw_counter") {
                if MotionModule::frame(fighter.module_accessor) >1.0 {
                    if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                        CancelModule::enable_cancel(fighter.module_accessor);
                    }
                }
            }   
        }
        //MIIBRAWLER
        /*if fighter_kind == *FIGHTER_KIND_MIISWORDSMAN {
            if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_lw1_catch") || MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_air_lw1_catch") {
                if MotionModule::frame(fighter.module_accessor) >1.0 {
                    if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                        CancelModule::enable_cancel(fighter.module_accessor);
                    }
                }
            }   
        }*/        
        //MIISWORDFIGHTER
        if fighter_kind == *FIGHTER_KIND_MIISWORDSMAN {
            if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_lw1_hit") || MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_air_lw1_hit") {
                if MotionModule::frame(fighter.module_accessor) >1.0 {
                    if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                        CancelModule::enable_cancel(fighter.module_accessor);
                    }
                }
            }   
        }
        //PALUTENA
        /*if fighter_kind == *FIGHTER_KIND_PALUTENA {
            if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_lw_attack") || MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_air_lw_attack") {
                if MotionModule::frame(fighter.module_accessor) >1.0 {
                    if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                        CancelModule::enable_cancel(fighter.module_accessor);
                    }
                }
            }   
        }*/
        //PEACH, DAISY
        /*if fighter_kind == *FIGHTER_KIND_PEACH
        || fighter_kind == *FIGHTER_KIND_DAISY {
            if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_n_hit") || MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_air_n_hit") {
                if MotionModule::frame(fighter.module_accessor) >1.0 {
                    COUNTER_CANCEL[entry_id] = true;
                }
            }   
        }
        //LITTLE MAC
        if fighter_kind == *FIGHTER_KIND_LITTLEMAC {
            if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_lw_hit") || MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_air_lw_hit") {
                if MotionModule::frame(fighter.module_accessor) >1.0 {
                    if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                        CancelModule::enable_cancel(fighter.module_accessor);
                    }
                }
            }   
        }        
        */

        //SORA
        if fighter_kind == *FIGHTER_KIND_DEMON+1 {
            if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_lw_rebound") || MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_air_lw_rebound") {
                if MotionModule::frame(fighter.module_accessor) >1.0 {
                    if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                        CancelModule::enable_cancel(fighter.module_accessor);
                    }
                }
            }   
        }  
        

    }                                      
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(once_per_fighter_frame);
}