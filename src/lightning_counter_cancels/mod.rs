use smash::app::lua_bind::StatusModule::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::lib::lua_const::*;
use acmd;

// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        //MARTH, LUCINA, ROY, CHROM, SEPHIROTH, CORRIN
        if (fighter_kind == *FIGHTER_KIND_MARTH 
        || fighter_kind == *FIGHTER_KIND_LUCINA
        || fighter_kind == *FIGHTER_KIND_ROY
        || fighter_kind == *FIGHTER_KIND_CHROM
        || fighter_kind == *FIGHTER_KIND_IKE
        || fighter_kind == *FIGHTER_KIND_EDGE
        || fighter_kind == *FIGHTER_KIND_KAMUI
        ) {

            if (MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw_hit") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw_hit")) {
                if MotionModule::frame(module_accessor) >1.0 {
                    if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }   
        }
        //SHULK
        if fighter_kind == *FIGHTER_KIND_SHULK {
            if (MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw_hit") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw_hit")) {
                if (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0) {
                    if (MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw")){
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }
        }
        //KROOL
        if fighter_kind == *FIGHTER_KIND_KROOL {
            if (MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw_hit") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw_hit")) {
                if MotionModule::frame(module_accessor) >2.0 {
                    if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }   
        }
        //LUCARIO
        if fighter_kind == *FIGHTER_KIND_LUCARIO {
            if MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw_split") {
                if MotionModule::frame(module_accessor) >1.0 {
                    if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }   
        }
        //GRENINJA
        if fighter_kind == *FIGHTER_KIND_GEKKOUGA {
            if (MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw_attack") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw_attack")) {
                if MotionModule::frame(module_accessor) >1.0 {
                    if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }   
        }
        //JOKER
        if fighter_kind == *FIGHTER_KIND_JACK {
            if (MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw_counter") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw_counter")) {
                if MotionModule::frame(module_accessor) >1.0 {
                    if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }   
        }
        //MIISWORDFIGHTER
        if fighter_kind == *FIGHTER_KIND_MIISWORDSMAN {
            if (MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw1_hit") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw1_hit")) {
                if MotionModule::frame(module_accessor) >1.0 {
                    if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }   
        }
        //PALUTENA
        if fighter_kind == *FIGHTER_KIND_PALUTENA {
            if (MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw_attack") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw_attack")) {
                if MotionModule::frame(module_accessor) >1.0 {
                    if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }   
        }
        //PEACH, DAISY
        if (fighter_kind == *FIGHTER_KIND_PEACH
        || fighter_kind == *FIGHTER_KIND_DAISY) {
            if (MotionModule::motion_kind(module_accessor) == smash::hash40("special_n_hit") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_n_hit")) {
                if MotionModule::frame(module_accessor) >1.0 {
                    if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }   
        }   
    }                                      
}

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
}