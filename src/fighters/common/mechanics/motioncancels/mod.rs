use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;
use crate::fighters::common::mechanics::lightning_mode::{LIGHTNING, LIGHTNING_BUTTON};
use crate::fighters::common::mechanics::vanish::{VANISH_COUNT, VANISH_COUNTER, CAN_VANISH, VANISH_BUTTON};
use crate::fighters::common::mechanics::ultrainstinct::{CROSS_CANCEL_BUTTON};
use crate::fighters::common::mechanics::crimson_cancel::{CRIMSON_CANCEL_BUTTON};
use crate::fighters::common::mechanics::lightning_fsmeter::{FINAL_SMASH_BUTTON};
use smash::phx::Hash40;
use smash::hash40;

//static mut MOTION_CHECK : [i32; 8] = [0; 8]; // Gets status kind while jump_guard_dash_upspecial_pressed. This is to avoid spam when u have no jumps/dodges left, so the status being checked would be the status being spammed. If it tdetects jump/dodge, it'll do nothing.
static mut CANCEL_IN_NEUTRAL : [bool; 8] = [false; 8];
static mut AIRDODGE_BUTTON : [bool; 8] = [false; 8];// for only running the code within it 1 frame.
static mut AIRDODGE_COUNT : [i32; 8] = [0; 8]; //  You start off with one airdodge. Every other airdodge after that before touching the ground increases the number up to how many jumps that fighter has.





#[fighter_frame_callback]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        //let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 0);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let motion_kind = MotionModule::motion_kind(module_accessor);       
        let frame = MotionModule::frame(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        //let lr = PostureModule::lr(module_accessor);

        let max_jumps = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        let edgde_one_wing_max_jumps = WorkModule::get_int(module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_JUMP_COUNT_MAX);
        let jumps_used = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        
        let grab = (motion_kind == smash::hash40("catch"))
        || (motion_kind == smash::hash40("catch_dash"))
        || (motion_kind == smash::hash40("catch_turn"));

        let l_stick_out = ControlModule::get_stick_x(fighter.module_accessor) > 0.7
        || ControlModule::get_stick_x(fighter.module_accessor) < -0.7 
        || ControlModule::get_stick_y(fighter.module_accessor) > 0.7 
        || ControlModule::get_stick_y(fighter.module_accessor) < -0.7;

        let special_mechanics_button = CRIMSON_CANCEL_BUTTON[entry_id]
        || CROSS_CANCEL_BUTTON[entry_id]
        || VANISH_BUTTON[entry_id]
        || LIGHTNING_BUTTON[entry_id];
        || FINAL_SMASH_BUTTON[entry_id];


        //AS MANY AIRDODGES AS YOU HAVE JUMPS

            //if LIGHTNING[entry_id] {

                if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                    if AIRDODGE_BUTTON[entry_id] == false {
                        AIRDODGE_COUNT[entry_id] +=1;
                        
                        AIRDODGE_BUTTON[entry_id] = true; // This is so the counter only runs one frame.
                        
                    }
                    if CANCEL_IN_NEUTRAL [entry_id] {
                        CANCEL_IN_NEUTRAL [entry_id] = false; // This is so cancel in neutral only runs before cancelling, to avoid spams.
                    }                  
                }
                else {
                    AIRDODGE_BUTTON[entry_id] = false;  
                }

                //Reset Airdodge count when u land
                if situation_kind == *SITUATION_KIND_GROUND { 
                    AIRDODGE_COUNT[entry_id] = 0;
                }


                if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL 
                || status_kind == *FIGHTER_STATUS_KIND_JUMP
                //|| characters that have more than 2 jumps don't work?
                || CANCEL_IN_NEUTRAL [entry_id] 
                || LIGHTNING [entry_id] && status_kind != *FIGHTER_STATUS_KIND_ESCAPE_AIR //in lightning, you can cancel directly into airdodge as many times as you have it available
                {//Account for all instances before inputing airdodge

                    if (ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_GUARD) //(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0  
                    && situation_kind == *SITUATION_KIND_AIR) 
                    && l_stick_out 
                    {
                        
                        if (max_jumps == 2 && AIRDODGE_COUNT[entry_id] <2)
                        || (max_jumps == 3 && AIRDODGE_COUNT[entry_id] <3) 
                        || (max_jumps == 4 && AIRDODGE_COUNT[entry_id] <4) 
                        || (max_jumps == 5 && AIRDODGE_COUNT[entry_id] <5) 
                        || (max_jumps == 6 && AIRDODGE_COUNT[entry_id] <6)
                        || (edgde_one_wing_max_jumps == 3 && AIRDODGE_COUNT[entry_id] <3)
                        {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                            
                        }  
                        CANCEL_IN_NEUTRAL [entry_id] = false; // This is so cancel in neutral only runs before cancelling, to avoid spams.
                    }
                    
                }                
            //}        
        //VANISHES

            //if SPECIAL_MECHANICS_METER_COUNT[entry_id] == 100 {
                if VANISH_COUNTER[entry_id] {
                   VANISH_COUNT[entry_id] += 1; 
                   VANISH_COUNTER[entry_id] = false;
                } 

                if LIGHTNING[entry_id] == false {//Vanish only once
                    if VANISH_COUNT[entry_id] <1 {
                        CAN_VANISH[entry_id] = true;
                    }     
                    else {
                        CAN_VANISH[entry_id] = false;
                    }               
                }

                else if LIGHTNING[entry_id] {// Vanish as many times as you can jump
                    if (max_jumps == 2 && VANISH_COUNT[entry_id] <2)
                    || (max_jumps == 3 && VANISH_COUNT[entry_id] <3) 
                    || (max_jumps == 4 && VANISH_COUNT[entry_id] <4) 
                    || (max_jumps == 5 && VANISH_COUNT[entry_id] <5) 
                    || (max_jumps == 6 && VANISH_COUNT[entry_id] <6)
                    || (edgde_one_wing_max_jumps == 3 && VANISH_COUNT[entry_id] <3)
                    {
                        CAN_VANISH[entry_id] = true; 
                    }
                    else {
                        CAN_VANISH[entry_id] = false;
                    }                    
                }  
            //}
            



        if CANCEL_IN_NEUTRAL [entry_id] {

            //CANCEL FOR AS MANY JUMPS AS YOU HAVE    
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
                    
                    if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false);
                    }
                    else if situation_kind == *SITUATION_KIND_AIR {

                        if (max_jumps == 2 && jumps_used <2)
                        || (max_jumps == 3 && jumps_used <3) 
                        || (max_jumps == 4 && jumps_used <4) 
                        || (max_jumps == 5 && jumps_used <5) 
                        || (max_jumps == 6 && jumps_used <6)
                        || (edgde_one_wing_max_jumps == 3 && jumps_used <3)
                        {
                            CancelModule::enable_cancel(fighter.module_accessor);
                        }
                    }
                }

            //EVERYTHING ELSE
                if (((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0) && situation_kind == *SITUATION_KIND_GROUND && ! grab) 
                || ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 && situation_kind == *SITUATION_KIND_AIR) 
                || special_mechanics_button
                {
                 
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
                CANCEL_IN_NEUTRAL [entry_id] = false;           
        }

        //RESETS
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
                AIRDODGE_COUNT[entry_id] = 0;
                VANISH_COUNT[entry_id] = 0;
                CAN_VANISH[entry_id] = true
            }         



        //
        if CANCEL_IN_NEUTRAL [entry_id] == false && ! AttackModule::is_attack_occur(fighter.module_accessor) {

            if fighter_kind == *FIGHTER_KIND_MARIO 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 7.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >7.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >25.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >27.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >21.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >7.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >23.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >17.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >17.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >14.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >14.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame >20.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >20.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >21.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >21.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=44.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=18.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=18.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {

                    CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                    
                } 
                    
            if fighter_kind == *FIGHTER_KIND_DONKEY
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") 
                    || motion_kind == smash::hash40("attack_s3_hi") 
                    || motion_kind == smash::hash40("attack_s3_lw")) && frame > 9.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame >11.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame >7.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame >24.0 )
                    || (motion_kind == smash::hash40("attack_s4") && frame >23.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame >16.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame >14.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame >26.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame >23.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame >16.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame >10.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame >16.0 )
                    || (motion_kind == smash::hash40("special_n") && frame >27.0 )
                    || (motion_kind == smash::hash40("special_s") && frame >21.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame >62.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >43.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame >24.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame >30.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=0.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=15.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=14.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=19.0 )
                    || (motion_kind == smash::hash40("catch") && frame >=10.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >13.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >14.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 

            if fighter_kind == *FIGHTER_KIND_LINK
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 19.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame >12.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame >12.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame >23.0 )
                    || (motion_kind == smash::hash40("attack_s4") && frame >18.0 )
                    || (motion_kind == smash::hash40("attack_s4_s2") && frame >11.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame >45.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame >25.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame >31.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame >25.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame >17.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame >40.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame >64.0 )
                    || (motion_kind == smash::hash40("link_special_n_end") && frame >18.0 )
                    || (motion_kind == smash::hash40("link_special_air_n_end") && frame >18.0 )
                    || (motion_kind == smash::hash40("special_s1") && frame >27.0 )
                    || (motion_kind == smash::hash40("special_air_s1") && frame >27.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame >39.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >49.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame >18.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame >18.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=14.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=28.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=24.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 

            if (fighter_kind == *FIGHTER_KIND_SAMUS || fighter_kind == *FIGHTER_KIND_SAMUSD)
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 10.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame >18.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame >8.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame >18.0 )
                    || (motion_kind == smash::hash40("attack_s4") && frame >11.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame >28.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame >18.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame >22.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame >31.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame >14.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame >17.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame >23.0 )
                    || (motion_kind == smash::hash40("air_catch") && frame >19.0 )
                    || ((motion_kind == smash::hash40("special_n_f") 
                    || motion_kind == smash::hash40("special_n_f_max")
                    || motion_kind == smash::hash40("special_air_n_f")
                    || motion_kind == smash::hash40("special_air_n_f_max")) && frame >1.0 )
                    || (motion_kind == smash::hash40("special_s") && frame >18.0 )
                    || (motion_kind == smash::hash40("special_air_s") && frame >18.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame >28.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >28.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame >21.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame >33.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=16.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=12.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=16.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=21.0 )
                    || (motion_kind == smash::hash40("catch") && frame >22.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >24.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >25.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
        
            if fighter_kind == *FIGHTER_KIND_YOSHI
                && (
                    ((motion_kind == smash::hash40("attack_s3_s")|| motion_kind == smash::hash40("attack_s3_hi")|| motion_kind == smash::hash40("attack_s3_lw")) && frame > 7.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame >15.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame >10.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame >20.0 )
                    || (motion_kind == smash::hash40("attack_s4") && frame >16.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame >16.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame >23.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame >25.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame >20.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame >19.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame >6.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame >41.0 )
                    || (motion_kind == smash::hash40("special_n") && frame >33.0 )
                    || (motion_kind == smash::hash40("special_air_n") && frame >33.0 )
                    || (motion_kind == smash::hash40("special_s") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_s") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame >16.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >16.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame >21.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=15.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=20.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=25.0 )
                    || (motion_kind == smash::hash40("catch") && frame >=21.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >=23.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >=24.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 

            if fighter_kind == *FIGHTER_KIND_KIRBY
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 8.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame >10.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame >6.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame >34.0 )
                    || (motion_kind == smash::hash40("attack_s4") && frame >19.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame >17.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame >19.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame >32.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame >27.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame >12.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame >13.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame >34.0 )
                    || (motion_kind == smash::hash40("special_n") && frame >44.0 )
                    || (motion_kind == smash::hash40("special_air_n") && frame >44.0 )
                    || (motion_kind == smash::hash40("special_s") && frame >27.0 )
                    || (motion_kind == smash::hash40("special_air_s") && frame >29.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=45.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=41.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=58.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=9.0 )
                    || (motion_kind == smash::hash40("catch") && frame >=7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 

            
            if fighter_kind == *FIGHTER_KIND_FOX
                && (
                    ((motion_kind == smash::hash40("attack_s3_s")|| motion_kind == smash::hash40("attack_s3_hi")|| motion_kind == smash::hash40("attack_s3_lw")) && frame > 8.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame > 7.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame > 8.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame > 15.0 )
                    || (motion_kind == smash::hash40("attack_s4") && frame > 14.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame > 11.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame > 7.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame > 23.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame > 27.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame > 11.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame > 13.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame > 23.0 )
                    || (motion_kind == smash::hash40("special_n_loop") && frame > 1.0 )
                    || (motion_kind == smash::hash40("special_air_n_loop") && frame >1.0 )
                    || (motion_kind == smash::hash40("special_s") && frame > 28.0 )
                    || (motion_kind == smash::hash40("special_air_s") && frame >28.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame >20.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >20.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=11.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=10.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=7.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=26.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >12.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            
            if fighter_kind == *FIGHTER_KIND_PIKACHU 
                && ((motion_kind == smash::hash40("attack_s3_s") && frame > 9.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame > 14.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame > 9.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame > 13.0 )
                    || (motion_kind == smash::hash40("attack_s4") && frame > 30.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame > 18.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame > 22.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame > 23.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame > 28.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame > 26.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame > 9.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame > 27.0 )
                    || (motion_kind == smash::hash40("special_n") && frame > 20.0 )
                    || (motion_kind == smash::hash40("special_air_n") && frame >20.0 )
                    || (motion_kind == smash::hash40("special_s") && frame > 53.0 )
                    || (motion_kind == smash::hash40("special_air_s") && frame >53.0 )
                    ||(motion_kind == smash::hash40("special_hi_end") && frame >10.0 )
                    ||(motion_kind == smash::hash40("special_air_hi_end") && frame >10.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame > 22.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=11.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=26.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=16.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=29.0 )
                    || (motion_kind == smash::hash40("catch") && frame >8.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >12.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >13.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }

            if fighter_kind == *FIGHTER_KIND_LUIGI 
                && ((motion_kind == smash::hash40("attack_s3_s") && frame > 7.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame > 11.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame > 9.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame > 26.0 )
                    || (motion_kind == smash::hash40("attack_s4") && frame > 14.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame > 14.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame > 16.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame > 32.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame > 11.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame > 14.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame > 12.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame > 15.0 )
                    || (motion_kind == smash::hash40("special_n") && frame > 18.0 )
                    || (motion_kind == smash::hash40("special_air_n") && frame >18.0 )
                    || (motion_kind == smash::hash40("special_s") && frame > 61.0 )
                    || (motion_kind == smash::hash40("special_air_s") && frame >61.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame >23.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >23.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame > 41.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame >41.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=15.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=15.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=18.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=29.0 )
                    || (motion_kind == smash::hash40("catch") && frame >20.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >22.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >24.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }

            if fighter_kind == *FIGHTER_KIND_NESS  
                && ((motion_kind == smash::hash40("attack_s3_s") && frame > 11.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame > 9.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame > 5.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame > 25.0 )
                    || (motion_kind == smash::hash40("attack_s4") && frame > 23.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame == 0.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame == 0.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame > 16.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame > 22.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame > 19.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame >17.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame > 25.0 )
                    || (motion_kind == smash::hash40("special_n") && frame > 122.0 )
                    || (motion_kind == smash::hash40("special_air_n") && frame >122.0 )
                    || (motion_kind == smash::hash40("special_s") && frame > 38.0 )
                    || (motion_kind == smash::hash40("special_air_s") && frame >38.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >33.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=27.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=27.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=36.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=29.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_CAPTAIN
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 12.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame > 17.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame > 12.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame > 16.0 )
                    || (motion_kind == smash::hash40("attack_s4") || motion_kind == smash::hash40("attack_s4_hi") || motion_kind == smash::hash40("attack_s4_lw") && frame > 30.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame > 29.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame > 30.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame > 15.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame > 30.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame > 15.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame > 11.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame > 18.0 )
                    || (motion_kind == smash::hash40("special_n") && frame > 66.0 )
                    || (motion_kind == smash::hash40("special_air_n") && frame >66.0 )
                    || (motion_kind == smash::hash40("special_s") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_s_start") && frame >44.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame >30.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >30.0 )
                    || (motion_kind == smash::hash40("special_lw_end") && frame > 1.0 )
                    || (motion_kind == smash::hash40("special_air_lw_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=14.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=21.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            //
            if fighter_kind == *FIGHTER_KIND_PURIN//JIGGLYPUFF//
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 10.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >24.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >30.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >21.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >32.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >28.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >28.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame >36.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >36.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >4.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >4.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=12.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=26.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=9.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=62.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if (fighter_kind == *FIGHTER_KIND_PEACH || fighter_kind == *FIGHTER_KIND_DAISY)
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 15.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >31.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >7.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >31.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >34.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >34.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_hi_start") && frame >32.0 )
                    ||(motion_kind == smash::hash40("special_air_hi_start") && frame >32.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=16.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=21.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=26.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=43.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_KOOPA
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 14.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >27.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >37.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >30.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >29.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >24.0 )
                    ||(motion_kind == smash::hash40("special_n_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_n_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame >39.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >51.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >37.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >37.0 )
                    ||(motion_kind == smash::hash40("special_lw_landing") && frame >1.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=36.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=19.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=49.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=37.0 )
                    || (motion_kind == smash::hash40("catch") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >13.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >14.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_NANA //ICECLIMBERS//
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 10.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >27.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >51.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >19.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >19.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >52.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >52.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >31.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >31.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=25.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=18.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=27.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=37.0 )
                    || (motion_kind == smash::hash40("catch") && frame >9.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_SHEIK 
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame >8.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >6.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >8.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >21.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >30.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >7.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >24.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >33.0 )
                    ||(motion_kind == smash::hash40("special_n_end") && frame > 26.0 )
                    ||(motion_kind == smash::hash40("special_air_n_end") && frame > 26.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame > 12.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame > 12.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame >3.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >3.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_lw_landing") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_lw_attack_landing") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_lw_return_landing") && frame >1.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=15.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=20.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=25.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >12.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_ZELDA
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 13.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >24.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >34.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >24.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >26.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >26.0 )
                    ||(motion_kind == smash::hash40("special_s_loop") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_s_loop") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_s_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_s_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >36.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame > 1.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame > 1.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=30.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=27.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=30.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=42.0 )
                    || (motion_kind == smash::hash40("catch") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >14.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >15.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_MARIOD 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 7.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >7.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >25.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >27.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >21.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >7.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >19.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >17.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >17.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >14.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >14.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame >19.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >19.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >41.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >41.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=40.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=18.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=18.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_PICHU
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 12.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >8.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >33.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >24.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >27.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >31.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >26.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >18.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >18.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >17.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >17.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame >10.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >10.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >16.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >16.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=27.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=26.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=15.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=19.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >9.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >10.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_FALCO 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 8.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >27.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >24.0 )
                    ||(motion_kind == smash::hash40("special_n_loop") && frame >8.0 )
                    ||(motion_kind == smash::hash40("special_air_n_loop") && frame >8.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame >20.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >20.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >14.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >14.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=12.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=9.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=7.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=33.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if (fighter_kind == *FIGHTER_KIND_MARTH || fighter_kind == *FIGHTER_KIND_LUCINA)
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 11.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >8.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >21.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >8.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >11.0 )
                    || (status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END && frame > 10.0)
                    || (status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX && frame > 10.0)
                    
                    ||(motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    || (status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2 && frame > 8.0)
                    || (status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3 && frame > 7.0)
                    ||(motion_kind == smash::hash40("special_s4_s") && frame >9.0 )
                    ||(motion_kind == smash::hash40("special_air_s4_s") && frame >9.0 )
                    || (motion_kind == smash::hash40("special_s4_hi") && frame >10.0 )
                    || (motion_kind == smash::hash40("special_air_s4_hi") && frame >10.0 )
                    || (motion_kind == smash::hash40("special_s4_lw") && frame >21.0 )
                    || (motion_kind == smash::hash40("special_air_s4_lw") && frame >21.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame >30.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >30.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=18.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=19.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=20.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }  
            //
            if fighter_kind == *FIGHTER_KIND_YOUNGLINK
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 12.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame > 14.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame > 9.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame > 10.0 )
                    || (motion_kind == smash::hash40("attack_s4") && frame > 16.0 )
                    || (motion_kind == smash::hash40("attack_s4_s2") && frame >12.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame > 45.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame > 22.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame > 27.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame > 25.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame > 20.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame > 49.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame > 64.0 )
                    || (motion_kind == smash::hash40("air_catch") && frame >16.0 )
                    || (motion_kind == smash::hash40("special_n") && frame > 14.0 )
                    || (motion_kind == smash::hash40("special_air_n") && frame >14.0 )
                    || (motion_kind == smash::hash40("special_s") && frame > 27.0 )
                    || (motion_kind == smash::hash40("special_air_s") && frame >27.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame >49.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >51.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame > 18.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame >18.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=30.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=28.0 )
                    || (motion_kind == smash::hash40("catch") && frame >18.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >20.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >21.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
        
            

            if fighter_kind == *FIGHTER_KIND_GANON
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 12.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame > 64.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame > 12.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame > 19.0 )
                    || (motion_kind == smash::hash40("attack_s4") && frame > 31.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame > 25.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame > 38.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame > 26.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame > 19.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame > 12.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame > 16.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame > 18.0 )
                    || (motion_kind == smash::hash40("ganon_special_n") && frame > 73.0 )
                    || (motion_kind == smash::hash40("ganon_special_n_turn") && frame > 83.0 )
                    || (motion_kind == smash::hash40("ganon_special_air_n") && frame >73.0 )
                    || (motion_kind == smash::hash40("ganon_special_air_n_turn") && frame > 83.0 )
                    || (motion_kind == smash::hash40("special_s_start") && frame > 30.0 )
                    || (motion_kind == smash::hash40("special_air_s_start") && frame >28.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame >36.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >36.0 )
                    || (motion_kind == smash::hash40("special_lw_end") && frame > 1.0 )
                    || (motion_kind == smash::hash40("special_air_lw_end_air") && frame >1.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=14.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=23.0 )
                    || (motion_kind == smash::hash40("catch") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >13.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >14.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }

            if fighter_kind == *FIGHTER_KIND_MEWTWO
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 11.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame > 13.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame > 7.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame > 20.0 )
                    || ((motion_kind == smash::hash40("attack_s4") 
                    || motion_kind == smash::hash40("attack_s4_hi") 
                    || motion_kind == smash::hash40("attack_s4_lw")) && frame > 22.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame > 25.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame > 23.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame > 28.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame > 9.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame > 17.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame > 14.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame > 18.0 )
                    || (motion_kind == smash::hash40("mewtwo_special_n_shoot") && frame > 6.0 )
                    || (motion_kind == smash::hash40("mewtwo_special_air_n_shoot") && frame >6.0 )
                    || (motion_kind == smash::hash40("special_s") && frame > 13.0 )
                    || (motion_kind == smash::hash40("special_air_s") && frame >13.0 )
                    ||(motion_kind == smash::hash40("special_hi") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_hi") && frame >22.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame > 23.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame >23.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=19.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=30.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=43.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=18.0 )
                    || (motion_kind == smash::hash40("catch") && frame >8.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >12.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
                
            if (fighter_kind == *FIGHTER_KIND_ROY || fighter_kind == *FIGHTER_KIND_CHROM)
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 10.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame > 11.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame > 8.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame > 16.0 )
                    || ((motion_kind == smash::hash40("attack_s4") 
                    || motion_kind == smash::hash40("attack_s4_hi") 
                    || motion_kind == smash::hash40("attack_s4_lw")) && frame > 14.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame > 23.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame > 22.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame > 21.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame > 12.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame > 10.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame > 12.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame > 17.0 )
                    || (motion_kind == smash::hash40("special_n_end") && frame > 14.0 )
                    || (motion_kind == smash::hash40("special_n_end_2") && frame > 14.0 )
                    || (motion_kind == smash::hash40("special_n_end_3") && frame > 14.0 )
                    || (motion_kind == smash::hash40("special_n_end_max") && frame > 15.0 )
                    || (motion_kind == smash::hash40("special_air_n_end") && frame >14.0 )
                    || (motion_kind == smash::hash40("special_air_n_end_2") && frame >14.0 )
                    || (motion_kind == smash::hash40("special_air_n_end_3") && frame >14.0 )
                    || (motion_kind == smash::hash40("special_air_n_end_max") && frame >15.0 )
                    || (status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2 && frame > 8.0)
                    || (status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3 && frame > 7.0)
                    || (motion_kind == smash::hash40("special_s4_s") && frame > 9.0 )
                    || (motion_kind == smash::hash40("special_air_s4_s") && frame >9.0 )
                    || (motion_kind == smash::hash40("special_s4_hi") && frame >10.0 )
                    || (motion_kind == smash::hash40("special_air_s4_hi") && frame >10.0 )
                    || (motion_kind == smash::hash40("special_s4_lw") && frame >21.0 )
                    || (motion_kind == smash::hash40("special_air_s4_lw") && frame >21.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame > 21.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >21.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame > 29.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame >29.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=15.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=8.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=16.0 )
                    || (motion_kind == smash::hash40("catch") && frame >8.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >12.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }

            if fighter_kind == *FIGHTER_KIND_GAMEWATCH
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame >20.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame > 22.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame > 8.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame > 19.0 )
                    || ((motion_kind == smash::hash40("attack_s4") 
                    || motion_kind == smash::hash40("attack_s4_hi") 
                    || motion_kind == smash::hash40("attack_s4_lw")) && frame > 18.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame > 25.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame > 16.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame > 23.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame > 13.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame > 22.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame > 41.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame > 38.0 )
                    || (motion_kind == smash::hash40("special_n") && frame > 20.0 )
                    || (motion_kind == smash::hash40("special_air_n") && frame >20.0 )
                    || (motion_kind == smash::hash40("special_s") && frame > 27.0 )
                    || (motion_kind == smash::hash40("special_air_s") && frame >27.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame > 23.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >23.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame > 30.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame >30.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=26.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=26.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=26.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=35.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }     
            if fighter_kind == *FIGHTER_KIND_METAKNIGHT 
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 6.0 )
                    ||(motion_kind == smash::hash40("attack_s3_s2") && frame > 2.0 )
                    ||(motion_kind == smash::hash40("attack_s3_s3") && frame > 2.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >4.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >24.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >21.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >6.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >4.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >46.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >46.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame > 48.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >48.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=10.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=18.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=46.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=74.0 )
                    || (motion_kind == smash::hash40("catch") && frame >8.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >12.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >13.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if (fighter_kind == *FIGHTER_KIND_PIT || fighter_kind == *FIGHTER_KIND_PITB)  
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 15.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >7.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >22.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >25.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >12.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >17.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >17.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame > 44.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >44.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >18.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >18.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=14.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=29.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=15.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=16.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_SZEROSUIT 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 8.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >28.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >29.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >24.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >30.0 )
                    ||(motion_kind == smash::hash40("air_catch") && frame >19.0 )
                    ||(motion_kind == smash::hash40("special_n_shoot") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_n_shoot") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >35.0 )
                    ||(motion_kind == smash::hash40("special_s2") && frame >48.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >35.0 )
                    ||(motion_kind == smash::hash40("special_air_s2") && frame >48.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame > 35.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >35.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw_kick") && frame >17.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=9.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=11.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=5.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=22.0 )
                    || (motion_kind == smash::hash40("catch") && frame >25.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >26.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >26.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_WARIO 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 15.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >5.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >27.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >42.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >21.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_hi_jump") && frame >32.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >26.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >26.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=20.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=48.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=25.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=30.0 )
                    || (motion_kind == smash::hash40("catch") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >13.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >14.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_SNAKE 
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 5.0 )
                    ||(motion_kind == smash::hash40("attack_s3_s2") && frame >8.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >7.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >43.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >35.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >22.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >38.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >26.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >26.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >26.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >9.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >9.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=21.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=20.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=23.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=35.0 )
                    || (motion_kind == smash::hash40("catch") && frame >9.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >12.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >10.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            //
            if fighter_kind == *FIGHTER_KIND_IKE 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 13.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >21.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >8.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >35.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >31.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >36.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >22.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >17.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >33.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >33.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=8.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=22.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=20.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=40.0 )
                    || (motion_kind == smash::hash40("catch") && frame >8.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >12.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_PZENIGAME 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 6.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >6.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >21.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >25.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >25.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >26.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >22.0 )
                    ||(motion_kind == smash::hash40("special_n_shot") && frame >40.0 )
                    ||(motion_kind == smash::hash40("special_air_n_shot") && frame >40.0 )
                    ||(motion_kind == smash::hash40("special_s_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame > 44.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >44.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=27.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=18.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=30.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >9.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >10.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_PFUSHIGISOU 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 22.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >6.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >19.0 )
                    ||((motion_kind == smash::hash40("attack_s4") || motion_kind == smash::hash40("attack_s4_hi") ||motion_kind == smash::hash40("attack_s4_lw")) && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >29.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >29.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >12.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >38.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >38.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >73.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >73.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=17.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=20.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=21.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=19.0 )
                    || (motion_kind == smash::hash40("catch") && frame >14.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >14.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >14.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_PLIZARDON 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 13.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >24.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >24.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >43.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >43.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >51.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >51.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame > 28.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >28.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=27.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=26.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=58.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=55.0 )
                    || (motion_kind == smash::hash40("catch") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >13.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >14.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_DIDDY 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 16.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >5.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >8.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >7.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >16.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_hi_jump") && frame > 44.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >20.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >20.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=21.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=18.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=19.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=22.0 )
                    || (motion_kind == smash::hash40("catch") && frame >8.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >12.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if motion_kind == smash::hash40("special_air_hi_jump") && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_LUCAS 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 9.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >4.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >54.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >41.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >26.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >22.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >36.0 )
                    ||(motion_kind == smash::hash40("air_catch") && frame >21.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >95.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >95.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >19.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >19.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >30.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=23.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=20.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=25.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=21.0 )
                    || (motion_kind == smash::hash40("catch") && frame >17.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >19.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >20.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_SONIC 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 11.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >7.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >30.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >38.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >32.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=11.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=43.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=23.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=42.0 )
                    || (motion_kind == smash::hash40("catch") && frame >8.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >12.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_DEDEDE
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 23.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >41.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >40.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >24.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >22.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >29.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >25.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >23.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >29.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >29.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >29.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=14.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=19.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=19.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=26.0 )
                    || (motion_kind == smash::hash40("catch") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >13.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >14.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_PIKMIN
                && (
                    (motion_kind == smash::hash40("attack_s3_s")  && frame > 17.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >24.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >16.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >9.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >9.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=19.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=22.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=23.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=32.0 )
                    || (motion_kind == smash::hash40("catch") && frame >23.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >23.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >23.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_LUCARIO
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 16.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >22.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >21.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >8.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >12.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >25.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_hi_end") && frame > 2.0 )
                    || (motion_kind == smash::hash40("special_air_hi_end") && frame >23.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >24.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >24.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=7.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=14.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=17.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=28.0 )
                    || (motion_kind == smash::hash40("catch") && frame >8.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >12.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }   
            if fighter_kind == *FIGHTER_KIND_ROBOT
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 9.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >7.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >3.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >8.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >32.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >8.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >32.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >24.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >26.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >25.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >25.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >44.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >44.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=11.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=12.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=58.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=50.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }  
            if fighter_kind == *FIGHTER_KIND_TOONLINK 
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 13.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >40.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >44.0 )
                    ||(motion_kind == smash::hash40("air_catch") && frame >20.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >18.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >18.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >27.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >27.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame > 48.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >48.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=14.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=18.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=28.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=24.0 )
                    || (motion_kind == smash::hash40("catch") && frame >18.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >20.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >21.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_WOLF 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 10.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >6.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >22.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >26.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >17.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >16.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >16.0 )
                    ||(motion_kind == smash::hash40("special_s_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_s_end") && frame >1.0 )
                    || (motion_kind == smash::hash40("special_hi_fall") && frame > 7.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >8.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >8.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=11.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=24.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=27.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=26.0 )
                    || (motion_kind == smash::hash40("catch") && frame >8.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_RUSH_END && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_MURABITO 
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 11.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >25.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >25.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >34.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >31.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >21.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >26.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >23.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >23.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=15.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=11.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=18.0 )
                    || (motion_kind == smash::hash40("catch") && frame >16.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >18.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >19.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            //
            if fighter_kind == *FIGHTER_KIND_ROCKMAN 
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 54.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >21.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >36.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >31.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >33.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >54.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >23.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >19.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >19.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=10.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=18.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=12.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=17.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_WIIFIT 
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 10.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >21.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >20.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >21.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >21.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >15.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >15.0 )
                    || (motion_kind == smash::hash40("special_hi_end") && frame > 3.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=38.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=21.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=28.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=24.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_ROSETTA 
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 9.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >8.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >33.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >22.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >32.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >22.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >22.0 )
                    || (motion_kind == smash::hash40("special_hi_end") && frame > 1.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >29.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >29.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=28.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=30.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=13.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_LITTLEMAC 
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 13.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >4.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >2.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >8.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_s_jump") && frame >31.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame > 26.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >26.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >27.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >27.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=17.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=19.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=12.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=16.0 )
                    || (motion_kind == smash::hash40("catch") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >14.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >15.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_GEKKOUGA
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 12.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >6.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >21.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >22.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >45.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >20.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >20.0 )
                    ||(motion_kind == smash::hash40("special_s_end_f") && frame >8.0 )
                    ||(motion_kind == smash::hash40("special_s_end_b") && frame >8.0 )
                    ||(motion_kind == smash::hash40("special_air_s_attack_f") && frame >8.0 )
                    ||(motion_kind == smash::hash40("special_air_s_attack_b") && frame >8.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=16.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=18.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=16.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=16.0 )
                    || (motion_kind == smash::hash40("catch") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >14.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >15.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_PALUTENA 
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 35.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >25.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >27.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >35.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >30.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >25.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >11.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >36.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >36.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >28.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >28.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame > 1.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >6.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >6.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=20.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=20.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=17.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=25.0 )
                    || (motion_kind == smash::hash40("catch") && frame >8.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >12.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_PACMAN 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 7.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >37.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >29.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >28.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >28.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >8.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >28.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >54.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >54.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_hi_loop") && frame >18.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >12.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >12.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=25.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=18.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=34.0 )
                    || (motion_kind == smash::hash40("catch") && frame >33.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >35.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >36.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_REFLET 
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 10.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >8.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >27.0 )
                    ||((motion_kind == smash::hash40("attack_hi4") || motion_kind == smash::hash40("attack_hi4_2")) && frame >32.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >28.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >28.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >24.0 )
                    ||(motion_kind == smash::hash40("special_n_shoot") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_n_shoot") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >17.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >17.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >65.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >65.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >18.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=16.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=25.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=15.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=16.0 )
                    || (motion_kind == smash::hash40("catch") && frame >8.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >12.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }    
            if fighter_kind == *FIGHTER_KIND_SHULK 
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 13.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >33.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >42.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >30.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >21.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >26.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >25.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_hi_add") && frame > 11.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >41.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >41.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >16.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >20.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >22.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >25.0 )
                    || (motion_kind == smash::hash40("catch") && frame >8.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >12.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_KOOPAJR 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 9.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >25.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >26.0 )
                    ||((motion_kind == smash::hash40("attack_s4") || motion_kind == smash::hash40("attack_s4_hi") ||motion_kind == smash::hash40("attack_s4_lw")) && frame >37.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >39.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >37.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >37.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >14.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >14.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=16.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=24.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=18.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=50.0 )
                    || (motion_kind == smash::hash40("catch") && frame >13.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >16.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >17.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_DUCKHUNT 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 11.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >7.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >30.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >29.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >29.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >37.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >21.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >21.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >17.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >17.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=18.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=15.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=18.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=24.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if (fighter_kind == *FIGHTER_KIND_RYU || fighter_kind == *FIGHTER_KIND_KEN) 
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 11.0 )
                    ||(motion_kind == smash::hash40("attack_s3_w") && frame > 19.0 )
                    ||(motion_kind == smash::hash40("attack_hi3_s") && frame > 6.0 )
                    ||(motion_kind == smash::hash40("attack_hi3_w") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_lw3_s") && frame >3.0 )
                    ||(motion_kind == smash::hash40("attack_lw3_w") && frame > 7.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >6.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >31.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >12.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >12.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >12.0 )
                    ||(motion_kind == smash::hash40("special_s_end") && frame >9.0 )
                    ||(motion_kind == smash::hash40("special_air_s_end") && frame >9.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame > 25.0 )
                    || (motion_kind == smash::hash40("special_hi_command") && frame > 25.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >25.0 )
                    || (motion_kind == smash::hash40("special_air_hi_command") && frame >25.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=16.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=23.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=25.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=19.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            //
            if fighter_kind == *FIGHTER_KIND_CLOUD 
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 10.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >28.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >25.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >39.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >18.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >18.0 )
                    ||(motion_kind == smash::hash40("special_n_lb") && frame >16.0 )
                    ||(motion_kind == smash::hash40("special_air_n_lb") && frame >16.0 )
                    //||(motion_kind == smash::hash40("special_s1") && frame >12.0 )
                    //||(motion_kind == smash::hash40("special_air_s1") && frame >12.0 )
                    //||(motion_kind == smash::hash40("special_s2") && frame >3.0 )
                    //||(motion_kind == smash::hash40("special_air_s2") && frame >3.0 )
                    ||(motion_kind == smash::hash40("special_s3") && frame >26.0 )
                    ||(motion_kind == smash::hash40("special_air_s3") && frame >26.0 )
                    ||(motion_kind == smash::hash40("special_s3_lb") && frame >39.0 )
                    ||(motion_kind == smash::hash40("special_air_s3_lb") && frame >39.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame > 27.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >27.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=9.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=15.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=15.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=18.0 )
                    || (motion_kind == smash::hash40("catch") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >12.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >14.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }  
            if fighter_kind == *FIGHTER_KIND_KAMUI 
                && (
                    (motion_kind == smash::hash40("attack_s3_s")&& frame > 9.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >7.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >28.0 )
                    ||(motion_kind == smash::hash40("special_n_fire") && frame >6.0 )
                    ||(motion_kind == smash::hash40("special_air_n_fire") && frame >6.0 )
                    ||(motion_kind == smash::hash40("special_n_end_1") && frame >10.0 )
                    ||(motion_kind == smash::hash40("special_air_n_end_1") && frame >10.0 )
                    ||(motion_kind == smash::hash40("special_n_end_2") && frame >10.0 )
                    ||(motion_kind == smash::hash40("special_air_n_end_2") && frame >10.0 )
                    ||(motion_kind == smash::hash40("special_n_end_1") && frame >10.0 )
                    ||(motion_kind == smash::hash40("special_air_n_end_1") && frame >10.0 )
                    ||(motion_kind == smash::hash40("special_n_end_2") && frame >10.0 )
                    ||(motion_kind == smash::hash40("special_air_n_end_2") && frame >10.0 )
                    ||(motion_kind == smash::hash40("special_s_jump") && frame >10.0 )
                    ||(motion_kind == smash::hash40("special_s_jump_landing") && frame > 1.0 )
                    ||(motion_kind == smash::hash40("special_s_attack_landing") && frame > 1.0 )
                    ||(motion_kind == smash::hash40("special_s_wall_jump") && frame >25.0 )
                    ||(motion_kind == smash::hash40("special_s_wall_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_s_wall_attack_f") && frame >24.0 )
                    ||(motion_kind == smash::hash40("special_s_wall_attack_b") && frame >24.0 )
                    ||(motion_kind == smash::hash40("special_s_attack_f_landing") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_s_attack_b_landing") && frame >1.0 )                                       
                    ||(motion_kind == smash::hash40("special_air_s") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame > 30.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >30.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >26.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >26.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=15.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=27.0 )
                    || (motion_kind == smash::hash40("catch") && frame >8.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >12.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_BAYONETTA 
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 12.0 )
                    ||(motion_kind == smash::hash40("attack_s3_s2") && frame > 12.0 )
                    ||(motion_kind == smash::hash40("attack_s3_s3") && frame > 15.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >8.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >26.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >4.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >8.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >25.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_air_f2") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_air_f3") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >35.0 )
                    ||(motion_kind == smash::hash40("special_n_end_f") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_n_end_h") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_n_end_f") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_n_end_h") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >39.0 )
                    ||(motion_kind == smash::hash40("special_air_s_u") && frame >19.0 )
                    ||(motion_kind == smash::hash40("special_air_s_d") && frame >25.0 )
                    ||(motion_kind == smash::hash40("special_air_s_d_landing") && frame >20.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >27.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >27.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=15.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=15.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=10.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=21.0 )
                    || (motion_kind == smash::hash40("catch") && frame >8.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >12.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_INKLING 
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 10.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >24.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >21.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >17.0 )
                    ||(motion_kind == smash::hash40("special_n_fire_n") && frame >24.0 )
                    ||(motion_kind == smash::hash40("special_air_n_fire_n") && frame >24.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame == 0.0 )
                    || (status_kind == *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_FALL && frame > 1.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_lw_min") && frame >12.0 )
                    ||(motion_kind == smash::hash40("special_lw_middle") && frame >12.0 )
                    ||(motion_kind == smash::hash40("special_lw_max") && frame >12.0 )
                    ||(motion_kind == smash::hash40("special_air_lw_min") && frame >12.0 )
                    ||(motion_kind == smash::hash40("special_air_lw_middle") && frame >12.0 )
                    ||(motion_kind == smash::hash40("special_air_lw_max") && frame >12.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=23.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=17.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=22.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=20.0 )
                    || (motion_kind == smash::hash40("catch") && frame >9.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_FALL && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_RIDLEY 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 11.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >21.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >27.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >40.0 )
                    ||(motion_kind == smash::hash40("special_n_faliure") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_n_faliure") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_s_start") && frame >34.0 )
                    ||(motion_kind == smash::hash40("special_air_s_start") && frame >34.0 )
                    ||(motion_kind == smash::hash40("special_s_drag_cliff") && frame >7.0 )
                    ||(motion_kind == smash::hash40("special_s_drag_wall") && frame >5.0 )
                    ||(motion_kind == smash::hash40("special_air_s_drag_jump") && frame >7.0 )
                    ||(motion_kind == smash::hash40("special_air_s_fall_jump") && frame >7.0 )
                    || (motion_kind == smash::hash40("special_hi_landing_lw") && frame > 10.0 )
                    || (motion_kind == smash::hash40("special_hi_landing_f") && frame > 10.0 )
                    || (motion_kind == smash::hash40("special_air_hi_charge_end_f") && frame >7.0 )
                    || (motion_kind == smash::hash40("special_air_hi_charge_end_b") && frame > 7.0 )
                    || (motion_kind == smash::hash40("special_air_hi_charge_end_hi") && frame >7.0 )
                    || (motion_kind == smash::hash40("special_air_hi_charge_end_lw") && frame > 7.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_lw_stab") && frame >31.0 )
                    ||(motion_kind == smash::hash40("special_air_lw_stab") && frame >31.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=18.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=19.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=25.0 )
                    || (motion_kind == smash::hash40("catch") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >13.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >14.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_END && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if (fighter_kind == *FIGHTER_KIND_SIMON || fighter_kind == *FIGHTER_KIND_RICHTER)
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 13.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >22.0 )
                    ||(motion_kind == smash::hash40("attack_lw3_2") && frame >28.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >24.0 )
                    ||((motion_kind == smash::hash40("attack_s4") || motion_kind == smash::hash40("attack_s4_hi") || motion_kind == smash::hash40("attack_s4_lw")) && frame >25.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >21.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >27.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >36.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >30.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >30.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >19.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >19.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame > 22.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >22.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >18.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >18.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=24.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=18.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=26.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=33.0 )
                    || (motion_kind == smash::hash40("catch") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >14.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >15.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_KROOL
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 13.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >25.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >25.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >30.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >21.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >30.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >30.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >27.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >27.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >28.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >28.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=28.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=32.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=67.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=40.0 )
                    || (motion_kind == smash::hash40("catch") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >13.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >14.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_SHIZUE
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 11.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >25.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >26.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >31.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >28.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >23.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >23.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >43.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >70.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_lw_set") && frame >49.0)
                    ||(motion_kind == smash::hash40("special_air_lw_fire") && frame >49.0)
                    ||(motion_kind == smash::hash40("throw_f") && frame >=15.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=14.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=20.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=18.0 )
                    || (motion_kind == smash::hash40("catch") && frame >16.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >17.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >19.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_GAOGAEN 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 14.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >25.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >21.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame >57.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame >57.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >32.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >32.0 )
                    || (motion_kind == smash::hash40("special_hi_start") && frame >25.0 )
                    || (motion_kind == smash::hash40("special_air_hi_start") && frame >25.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame >27.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame >27.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=58.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=30.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=27.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=22.0 )
                    || (motion_kind == smash::hash40("catch") && frame >9.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >13.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >14.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            if fighter_kind == *FIGHTER_KIND_PACKUN 
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 8.0 )
                    ||(motion_kind == smash::hash40("attack_s3_2") && frame >6.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >14.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >21.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >21.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame > 67.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >67.0 )
                    ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=14.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=19.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=20.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=31.0 )
                    || (motion_kind == smash::hash40("catch") && frame >33.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >35.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >36.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                } 
            // 
            if fighter_kind == *FIGHTER_KIND_JACK
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 19.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame >23.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame >14.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame >21.0 )
                    || (motion_kind == smash::hash40("attack_s4") && frame >19.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame >14.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame >17.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame >27.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame >14.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame >8.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame >21.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame >16.0 )
                    || (motion_kind == smash::hash40("special_n") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_s1") && frame >16.0 )
                    || (motion_kind == smash::hash40("special_air_s1") && frame >16.0 )
                    || (motion_kind == smash::hash40("special_s2") && frame >16.0 )
                    || (motion_kind == smash::hash40("special_air_s2") && frame >16.0 )
                    || (motion_kind == smash::hash40("special_air_hi_f") && frame > 37.0 )
                    || (motion_kind == smash::hash40("special_air_hi_b") && frame >37.0 )
                    || (motion_kind == smash::hash40("special_lw_end") && frame >1.0 )
                    || (motion_kind == smash::hash40("special_air_lw_end") && frame >1.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame >31.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame >31.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=9.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=14.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=16.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=23.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >9.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >10.0)
                ) {
                 
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_BRAVE
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 11.0 )
                    || (motion_kind == smash::hash40("attack_s3_s2") && frame > 9.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame >11.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame >10.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame >23.0 )
                    || (motion_kind == smash::hash40("attack_s4") && frame >19.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame >17.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame >21.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame >16.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame >17.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame >20.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame >10.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame >24.0 )
                    || (motion_kind == smash::hash40("special_n_1") && frame >10.0 )
                    || (motion_kind == smash::hash40("special_air_n_1") && frame >10.0 )
                    || (motion_kind == smash::hash40("special_n_2") && frame >11.0 )
                    || (motion_kind == smash::hash40("special_air_n_2") && frame >11.0 )
                    || (motion_kind == smash::hash40("special_n_3") && frame >16.0 )
                    || (motion_kind == smash::hash40("special_air_n_3") && frame >16.0 )
                    || (motion_kind == smash::hash40("special_s_1") && frame >9.0 )
                    || (motion_kind == smash::hash40("special_air_s_1") && frame >9.0 )
                    || (motion_kind == smash::hash40("special_s_2") && frame >10.0 )
                    || (motion_kind == smash::hash40("special_air_s_2") && frame >10.0 )
                    || (motion_kind == smash::hash40("special_s_3") && frame >43.0 )
                    || (motion_kind == smash::hash40("special_air_s_3") && frame >43.0 )
                    || (motion_kind == smash::hash40("special_hi1") && frame > 40.0 )
                    || (motion_kind == smash::hash40("special_air_hi1") && frame >40.0 )
                    || (motion_kind == smash::hash40("special_hi2") && frame > 60.0 )
                    || (motion_kind == smash::hash40("special_air_hi2") && frame >60.0 )
                    || (motion_kind == smash::hash40("special_hi3") && frame > 90.0 )
                    || (motion_kind == smash::hash40("special_air_hi3") && frame >90.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=16.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=18.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=17.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=19.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                    
                    || (motion_kind == smash::hash40("special_lw1") && frame >6.0 )
                    || (motion_kind == smash::hash40("special_air_lw1") && frame >6.0 )

                    || (motion_kind == smash::hash40("special_lw2") && frame >6.0 )
                    || (motion_kind == smash::hash40("special_air_lw2") && frame >6.0 )

                    || (motion_kind == smash::hash40("special_lw3") && frame >6.0 )
                    || (motion_kind == smash::hash40("special_air_lw3") && frame >6.0 )

                    || (motion_kind == smash::hash40("special_lw4") && frame >6.0 )
                    || (motion_kind == smash::hash40("special_air_lw4") && frame >6.0 )

                    || (motion_kind == smash::hash40("special_lw5") && frame >6.0 )
                    || (motion_kind == smash::hash40("special_air_lw5") && frame >6.0 )
                    
                    || (motion_kind == smash::hash40("special_lw6") && frame >6.0 )
                    || (motion_kind == smash::hash40("special_air_lw6") && frame >6.0 )

                    || (motion_kind == smash::hash40("special_lw7") && frame >7.0 )
                    || (motion_kind == smash::hash40("special_air_lw7") && frame >7.0 )

                    || (motion_kind == smash::hash40("special_lw8") && frame >93.0 )
                    || (motion_kind == smash::hash40("special_air_lw8") && frame >93.0 )

                    || (motion_kind == smash::hash40("special_lw9") && frame >22.0 )
                    || (motion_kind == smash::hash40("special_air_lw9") && frame >22.0 )

                    || (motion_kind == smash::hash40("special_lw10") && frame >6.0 )
                    || (motion_kind == smash::hash40("special_air_lw10") && frame >6.0 )

                    || (motion_kind == smash::hash40("special_lw11") && frame >11.0 )
                    || (motion_kind == smash::hash40("special_air_lw11") && frame >11.0 )

                    || (motion_kind == smash::hash40("special_lw12") && frame >11.0 )
                    || (motion_kind == smash::hash40("special_air_lw12") && frame >11.0 )

                    || (motion_kind == smash::hash40("special_lw13") && frame >11.0 )
                    || (motion_kind == smash::hash40("special_air_lw13") && frame >11.0 )
                    
                    || (motion_kind == smash::hash40("special_lw14") && frame >6.0 )
                    || (motion_kind == smash::hash40("special_air_lw14") && frame >6.0 )

                    || (motion_kind == smash::hash40("special_lw15") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_lw15") && frame == 0.0 )

                    || (motion_kind == smash::hash40("special_lw16") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_lw16") && frame == 0.0 )

                    || (motion_kind == smash::hash40("special_lw17") && frame >11.0 )
                    || (motion_kind == smash::hash40("special_air_lw17") && frame >11.0 )

                    || (motion_kind == smash::hash40("special_lw18") && frame >11.0 )
                    || (motion_kind == smash::hash40("special_air_lw18") && frame >11.0 )

                    || (motion_kind == smash::hash40("special_lw19") && frame >12.0 )
                    || (motion_kind == smash::hash40("special_air_lw19") && frame >12.0 )

                    || (motion_kind == smash::hash40("special_lw20") && frame >40.0 )
                    || (motion_kind == smash::hash40("special_air_lw20") && frame >40.0 )

                    || (motion_kind == smash::hash40("special_lw21") && frame >6.0 )
                    || (motion_kind == smash::hash40("special_air_lw21") && frame >6.0 )

                ) {
                 
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_BUDDY
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 9.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame >14.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame >21.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame >20.0 )
                    || (motion_kind == smash::hash40("attack_s4") && frame >21.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame >27.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame >17.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame >32.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame >18.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame >17.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame >11.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame >45.0 )
                    || (motion_kind == smash::hash40("special_n_end") && frame >13.0 )
                    || (motion_kind == smash::hash40("special_air_n_end") && frame >13.0 )
                    || (motion_kind == smash::hash40("special_s") && frame >53.0 )
                    || (motion_kind == smash::hash40("special_air_s") && frame >53.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame >10.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame >10.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=11.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=36.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=15.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=34.0 )
                    || (motion_kind == smash::hash40("catch") && frame >8.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >12.0)
                ) {
                 
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_DOLLY
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 13.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame >11.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame >9.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame >22.0 )
                    || (motion_kind == smash::hash40("attack_s4") && frame >21.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame >14.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame >11.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame >19.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame >17.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame >15.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame >9.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame >15.0 )
                    || (motion_kind == smash::hash40("special_n") && frame >21.0 )
                    || (motion_kind == smash::hash40("special_air_n") && frame >18.0 )
                    || (motion_kind == smash::hash40("special_f_end") && frame >1.0 )
                    || (motion_kind == smash::hash40("special_air_f_end") && frame >1.0 )
                    || (motion_kind == smash::hash40("special_b_landing") && frame >1.0 )
                    || (motion_kind == smash::hash40("special_b_attack") && frame >17.0 )
                    || (motion_kind == smash::hash40("special_b_attack_w") && frame >17.0 )
                    || (motion_kind == smash::hash40("special_lw_landing") && frame >1.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame >41.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame > 28.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >28.0 )
                    || (motion_kind == smash::hash40("special_hi_command") && frame > 34.0 )
                    || (motion_kind == smash::hash40("special_air_hi_command") && frame >34.0 )
                    || (motion_kind == smash::hash40("super_special") && frame >20.0 )
                    || (motion_kind == smash::hash40("super_special2_start") && frame >30.0 )
                    || (motion_kind == smash::hash40("super_special2_blow") && frame >15.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=21.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=21.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=23.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                 
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                    if (motion_kind == smash::hash40("special_hi") 
                    || motion_kind == smash::hash40("special_air_hi") 
                    || motion_kind == smash::hash40("special_hi_command")
                    || motion_kind == smash::hash40("special_air_hi_command")) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                    
                }
            if fighter_kind == *FIGHTER_KIND_MASTER
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 10.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame >15.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame >15.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame >11.0 )
                    || ((motion_kind == smash::hash40("attack_s4") || motion_kind == smash::hash40("attack_s4_hi") || motion_kind == smash::hash40("attack_s4_lw")) && frame >25.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame >29.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame >31.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame >28.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame >13.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame >17.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame >23.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame >24.0 )
                    || (motion_kind == smash::hash40("special_n") && frame >1.0 )
                    || (motion_kind == smash::hash40("special_n_max") && frame >1.0 )
                    || (motion_kind == smash::hash40("special_air_n") && frame >1.0 )
                    || (motion_kind == smash::hash40("special_s_front_dash") && frame >24.0 )
                    || (motion_kind == smash::hash40("special_air_s_front_dash") && frame >24.0 )
                    || (motion_kind == smash::hash40("special_s_front") && frame >24.0 )
                    || (motion_kind == smash::hash40("special_air_s_front") && frame >24.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame >67.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame >67.0 )
                    || (motion_kind == smash::hash40("special_lw_hit") && frame >67.0 )
                    || (motion_kind == smash::hash40("special_air_lw_hit") && frame >67.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=30.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=15.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=13.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >12.0)
                ) {
                 
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_TANTAN
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame == 0.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame >15.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame >19.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame >16.0 )
                    || (motion_kind == smash::hash40("attack_s4") && frame == 0.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame >16.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame >8.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame >32.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame == 0.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame == 0.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame >13.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame >33.0 )
                    || (motion_kind == smash::hash40("special_n") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_s") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_s") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=31.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=11.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=16.0 )
                    || (motion_kind == smash::hash40("catch") && frame >24.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >24.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >24.0)
                ) {
                 
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_PICKEL
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 6.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame >9.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame >12.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame >13.0 )
                    || (motion_kind == smash::hash40("attack_s4") && frame >15.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame >8.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame >35.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame >6.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame >12.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame >16.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame >8.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame >8.0 )
                    || (motion_kind == smash::hash40("special_n") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_s") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_s") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >60.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=14.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=24.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=19.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=20.0 )
                    || (motion_kind == smash::hash40("catch") && frame >27.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >30.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >31.0)
                ) {
                 
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_EDGE
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 16.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame >25.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame >22.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame >19.0 )
                    || (motion_kind == smash::hash40("attack_s4") && frame >25.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame >28.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame >22.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame >11.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame >18.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame >16.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame >21.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame >39.0 )
                    || (motion_kind == smash::hash40("special_n_start") && frame >120.0 )
                    || (motion_kind == smash::hash40("special_n1") && frame >13.0 )
                    || (motion_kind == smash::hash40("special_n2") && frame >13.0 )
                    || (motion_kind == smash::hash40("special_air_n_start") && frame >120.0 )
                    || (motion_kind == smash::hash40("special_air_n1") && frame >13.0 )
                    || (motion_kind == smash::hash40("special_air_n2") && frame >13.0 )
                    || (motion_kind == smash::hash40("special_s") && frame >8.0 )
                    || (motion_kind == smash::hash40("special_air_s") && frame >8.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame >6.0 )
                    || (motion_kind == smash::hash40("special_air_hi2_end") && frame > 1.0 )
                    || (motion_kind == smash::hash40("special_air_hi1_end") && frame > 1.0 )
                    || (motion_kind == smash::hash40("special_lw") && frame >27.0 )
                    || (motion_kind == smash::hash40("special_air_lw") && frame >27.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=14.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=18.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=14.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=40.0 )
                    || (motion_kind == smash::hash40("catch") && frame >8.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >12.0)
                ) {
                 
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_EFLAME
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 13.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >33.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >22.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >20.0 )
                    ||(motion_kind == smash::hash40("special_n4") && frame >12.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >14.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >14.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame == 0.0 )
                    || (motion_kind == smash::hash40("special_air_hi_fall") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_lw_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_lw_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=11.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=17.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=9.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=28.0 )
                    || (motion_kind == smash::hash40("catch") && frame >8.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >13.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >14.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_ELIGHT
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 9.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >7.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >27.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >16.0 )
                    ||(motion_kind == smash::hash40("special_n") && frame > 37.0 )
                    ||(motion_kind == smash::hash40("special_air_n") && frame > 37.0 )
                    ||(motion_kind == smash::hash40("special_n2") && frame > 46.0 )
                    ||(motion_kind == smash::hash40("special_air_n2") && frame > 46.0 )
                    ||(motion_kind == smash::hash40("special_s_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_s_end") && frame >1.0 )
                    || (motion_kind == smash::hash40("special_hi_end") && frame >1.0 )
                    || (motion_kind == smash::hash40("special_air_hi_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_lw_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_lw_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=11.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=17.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=9.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=28.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >12.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >13.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_FINISH && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_DEMON
            && (
                ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 16.0 )
                ||(motion_kind == smash::hash40("attack_hi3") && frame >10.0 )
                ||(motion_kind == smash::hash40("attack_hi3_2") && frame >16.0 )
                ||(motion_kind == smash::hash40("attack_lw3") && frame >18.0 )
                ||(motion_kind == smash::hash40("attack_dash") && frame >19.0 )
                ||(motion_kind == smash::hash40("attack_s4") && frame >26.0 )
                ||(motion_kind == smash::hash40("attack_s4_transform") && frame >26.0 )
                ||(motion_kind == smash::hash40("attack_hi4") && frame >16.0 )
                ||(motion_kind == smash::hash40("attack_hi4_transform") && frame >22.0 )
                ||(motion_kind == smash::hash40("attack_lw4") && frame >19.0 )
                ||(motion_kind == smash::hash40("attack_lw4_transform") && frame >22.0 )
                ||(motion_kind == smash::hash40("attack_air_n") && frame >16.0 )
                ||(motion_kind == smash::hash40("attack_air_f") && frame >14.0 )
                ||(motion_kind == smash::hash40("attack_air_b") && frame >18.0 )
                ||(motion_kind == smash::hash40("attack_air_hi") && frame >09.0 )
                ||(motion_kind == smash::hash40("attack_air_lw") && frame >44.0 )
                ||(motion_kind == smash::hash40("special_n") && frame >11.0 )
                ||(motion_kind == smash::hash40("special_n_hi") && frame >11.0 )
                ||(motion_kind == smash::hash40("special_n_lw") && frame >11.0 )
                ||(motion_kind == smash::hash40("special_air_n") && frame >11.0 )
                ||(motion_kind == smash::hash40("special_air_n_hi") && frame >11.0 )
                ||(motion_kind == smash::hash40("special_air_n_lw") && frame >11.0 )
                ||(motion_kind == smash::hash40("special_s_end") && frame >1.0 )
                ||(motion_kind == smash::hash40("special_air_s_end") && frame >1.0 )
                ||(motion_kind == smash::hash40("special_lw") && frame == 0.0 )
                ||(motion_kind == smash::hash40("special_air_lw") && frame == 0.0 )
                ||(motion_kind == smash::hash40("throw_f") && frame >=42.0 )
                ||(motion_kind == smash::hash40("throw_b") && frame >=46.0 )
                ||(motion_kind == smash::hash40("throw_hi") && frame >=14.0 )
                ||(motion_kind == smash::hash40("throw_lw") && frame >=35.0 )
                || (motion_kind == smash::hash40("catch") && frame >8.0)
                || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ||(motion_kind == smash::hash40("attack_stand_1") && frame >14.0 )
                ||(motion_kind == smash::hash40("attack_stand_21") && frame >20.0 )
                ||(motion_kind == smash::hash40("attack_stand_22") && frame >11.0 )
                ||(motion_kind == smash::hash40("attack_stand_23") && frame >16.0 )
                ||(motion_kind == smash::hash40("attack_stand_24") && frame >15.0 )
                ||(motion_kind == smash::hash40("attack_stand_31") && frame >10.0 )
                ||(motion_kind == smash::hash40("attack_stand_4") && frame >14.0 )
                ||(motion_kind == smash::hash40("attack_stand_5") && frame >12.0 )
                ||(motion_kind == smash::hash40("attack_stand_6") && frame >16.0 )
                ||(motion_kind == smash::hash40("attack_squat_1") && frame >18.0 )
                ||(motion_kind == smash::hash40("attack_squat_2") && frame >14.0 )
                ||(motion_kind == smash::hash40("attack_squat_3") && frame >15.0 )
                ||(motion_kind == smash::hash40("attack_squat_4") && frame >13.0 )
                ||(motion_kind == smash::hash40("attack_step_2") && frame >13.0 )
                ||(motion_kind == smash::hash40("attack_step_2f") && frame >14.0 )
                ||(motion_kind == smash::hash40("attack_step_2l") && frame >27.0 )
                ||(motion_kind == smash::hash40("attack_step_2s") && frame >35.0 )

            ) {
                 
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
            }
            if fighter_kind == *FIGHTER_KIND_DEMON + 1
                && (
                    (motion_kind == smash::hash40("attack_s3") && frame > 17.0 )
                    ||(motion_kind == smash::hash40("attack_s3_2") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_s3_3") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >36.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >24.0 )
                    ||(motion_kind == smash::hash40("attack_s4") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >19.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >22.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_air_n2") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_air_n3") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_air_f2") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_air_f3") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >44.0 )
                    ||(motion_kind == smash::hash40("special_n1") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_n1") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_n2") && frame >38.0 )
                    ||(motion_kind == smash::hash40("special_air_n2") && frame >38.0 )
                    ||(motion_kind == smash::hash40("special_n3") && (frame == 27.0 || frame == 41.0 || frame > 55.0))
                    ||(motion_kind == smash::hash40("special_air_n3") && (frame == 27.0 || frame == 41.0 || frame > 55.0))
                    ||(motion_kind == smash::hash40("special_air_n") && frame == 0.0 )
                    ||(motion_kind == smash::hash40("special_s") && frame >14.0 )
                    ||(motion_kind == smash::hash40("special_air_s") && frame >14.0 )
                    || (motion_kind == smash::hash40("special_hi") && frame > 43.0 )
                    || (motion_kind == smash::hash40("special_air_hi") && frame >43.0 )
                    ||(motion_kind == smash::hash40("special_lw_start") && frame >25.0 )
                    ||(motion_kind == smash::hash40("special_air_lw_start") && frame >25.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=16.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=11.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=19.0 )
                    || (motion_kind == smash::hash40("catch") && frame >8.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >11.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >12.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_MIIFIGHTER
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 8.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >8.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >17.0 )
                    ||((motion_kind == smash::hash40("attack_s4") || motion_kind == smash::hash40("attack_s4_hi") ||motion_kind == smash::hash40("attack_s4_lw")) && frame >18.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >14.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >29.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >18.0 )
                    ||(motion_kind == smash::hash40("special_n1") && frame >28.0 )
                    ||(motion_kind == smash::hash40("special_air_n1") && frame >28.0 )
                    ||(motion_kind == smash::hash40("special_n2_miss") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_n2_miss") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_n3") && frame >52.0 )
                    ||(motion_kind == smash::hash40("special_air_n3") && frame >52.0 )
                    ||(motion_kind == smash::hash40("special_s_start") && frame >31.0 )
                    ||(motion_kind == smash::hash40("special_air_s_start") && frame >31.0 )
                    ||(motion_kind == smash::hash40("special_s2_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_s2_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_s3_dash") && frame >13.0 )
                    ||(motion_kind == smash::hash40("special_air_s3_dash") && frame >13.0 )
                    ||(motion_kind == smash::hash40("special_hi1_2") && frame >26.0 )
                    ||(motion_kind == smash::hash40("special_air_hi1_2") && frame >26.0 ) 
                    ||(motion_kind == smash::hash40("special_hi2") && frame >38.0 )
                    ||(motion_kind == smash::hash40("special_air_hi2") && frame >38.0 )  
                    ||(motion_kind == smash::hash40("special_hi3") && frame >37.0 )
                    ||(motion_kind == smash::hash40("special_air_hi3") && frame >37.0 )         
                    ||(motion_kind == smash::hash40("special_hi4") && frame >22.0 )
                    ||(motion_kind == smash::hash40("special_air_hi4") && frame >22.0 ) 
                    ||(motion_kind == smash::hash40("special_lw2_kick_landing") && frame >1.0 ) 
                    ||(motion_kind == smash::hash40("special_lw2_landing") && frame >1.0 ) 
                    ||(motion_kind == smash::hash40("special_air_lw2_landing") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_lw3") && frame >23.0 )
                    ||(motion_kind == smash::hash40("special_air_lw3") && frame >23.0 )
                    ||(motion_kind == smash::hash40("throw_f") && frame >=14.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=11.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=16.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=20.0 )
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                    if (motion_kind == smash::hash40("special_hi1_2") && frame >26.0 )
                    ||(motion_kind == smash::hash40("special_air_hi1_2") && frame >26.0 )     
                    ||(motion_kind == smash::hash40("special_hi2") && frame >38.0 )
                    ||(motion_kind == smash::hash40("special_air_hi2") && frame >38.0 )  
                    ||(motion_kind == smash::hash40("special_hi3") && frame >37.0 )
                    ||(motion_kind == smash::hash40("special_air_hi3") && frame >37.0 )         
                    ||(motion_kind == smash::hash40("special_hi4") && frame >22.0 )
                    ||(motion_kind == smash::hash40("special_air_hi4") && frame >22.0 )
                        && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_MIIGUNNER
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 12.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >9.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >14.0 )
                    ||((motion_kind == smash::hash40("attack_s4") || motion_kind == smash::hash40("attack_s4_hi") ||motion_kind == smash::hash40("attack_s4_lw")) && frame >40.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >28.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >25.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >21.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >20.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >34.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >25.0 )
                    ||(motion_kind == smash::hash40("special_n1_fire") && frame >3.0 )
                    ||(motion_kind == smash::hash40("special_n1_fire_max") && frame >3.0 )
                    ||(motion_kind == smash::hash40("special_air_n1_fire") && frame >3.0 )
                    ||(motion_kind == smash::hash40("special_air_n1_fire_max") && frame >3.0 )
                    ||(motion_kind == smash::hash40("special_n2_loop") && frame >4.0 )
                    ||(motion_kind == smash::hash40("special_air_n2_loop") && frame >4.0 )
                    ||(motion_kind == smash::hash40("special_n3_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_n3_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_s1") && frame >21.0 )
                    ||(motion_kind == smash::hash40("special_air_s1") && frame >21.0 )
                    ||(motion_kind == smash::hash40("special_s2_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_s2_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_s3_1") && frame >23.0 )
                    ||(motion_kind == smash::hash40("special_air_s3_1") && frame >23.0 )
                    ||(motion_kind == smash::hash40("special_s3_2") && frame >23.0 )
                    ||(motion_kind == smash::hash40("special_air_s3_2") && frame >23.0 )

                    ||(motion_kind == smash::hash40("special_hi1") && frame >25.0)
                    ||(motion_kind == smash::hash40("special_air_hi1") && frame >25.0)
                    ||(motion_kind == smash::hash40("special_hi2_squat") && frame == 13.0 ) || (motion_kind == smash::hash40("special_hi1") && frame >25.0 )
                    ||(motion_kind == smash::hash40("special_air_hi2_squat") && frame == 13.0 ) || (motion_kind == smash::hash40("special_hi1") && frame >25.0 )         
                    ||(motion_kind == smash::hash40("special_hi3") && frame >50.0 )
                    ||(motion_kind == smash::hash40("special_air_hi3") && frame >50.0 )  

                    ||(motion_kind == smash::hash40("special_lw1_start") && frame >4.0 )
                    ||(motion_kind == smash::hash40("special_air_lw1_start") && frame >4.0 )
                    ||(motion_kind == smash::hash40("special_lw3_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_lw3_end") && frame >1.0 )  
                    ||(motion_kind == smash::hash40("throw_f") && frame >=12.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=10.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=7.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=16.0 )          
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI2_JUMP || status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
            if fighter_kind == *FIGHTER_KIND_MIISWORDSMAN
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 11.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >6.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >10.0 )
                    ||((motion_kind == smash::hash40("attack_s4") || motion_kind == smash::hash40("attack_s4_hi") ||motion_kind == smash::hash40("attack_s4_lw")) && frame >17.0 )
                    ||(motion_kind == smash::hash40("attack_hi4") && frame >22.0 )
                    ||(motion_kind == smash::hash40("attack_lw4") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_air_n") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_air_f") && frame >22.0 )
                    ||(motion_kind == smash::hash40("attack_air_b") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_air_hi") && frame >23.0 )
                    ||(motion_kind == smash::hash40("attack_air_lw") && frame >37.0 )
                    ||(motion_kind == smash::hash40("special_n1") && frame >20.0 )
                    ||(motion_kind == smash::hash40("special_air_n1") && frame >20.0 )
                    ||(motion_kind == smash::hash40("special_n2") && frame >13.0 )
                    ||(motion_kind == smash::hash40("special_air_n2") && frame >13.0 )
                    ||(motion_kind == smash::hash40("special_n3_end") && frame >35.0 )
                    ||(motion_kind == smash::hash40("special_air_n3_end") && frame >35.0 )
                    ||(motion_kind == smash::hash40("special_s1_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_s1_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_s2_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_air_s2_end") && frame >1.0 )
                    ||(motion_kind == smash::hash40("special_s3_1") && frame >18.0 )
                    ||(motion_kind == smash::hash40("special_air_s3_1") && frame >18.0 )
                    ||(motion_kind == smash::hash40("special_s3_1_hi") && frame >18.0 )
                    ||(motion_kind == smash::hash40("special_air_s3_1_hi") && frame >18.0 )
                    ||(motion_kind == smash::hash40("special_s3_1_lw") && frame >18.0 )
                    ||(motion_kind == smash::hash40("special_air_s3_1_lw") && frame >18.0 )

                    ||(motion_kind == smash::hash40("special_hi1") && frame >44.0 )
                    ||(motion_kind == smash::hash40("special_air_hi1") && frame >44.0 )
                    ||(motion_kind == smash::hash40("special_hi2") && frame >21.0 )
                    ||(motion_kind == smash::hash40("special_air_hi2") && frame >36.0 )         
                    ||(motion_kind == smash::hash40("special_hi3") && frame >36.0 )
                    ||(motion_kind == smash::hash40("special_air_hi3") && frame >49.0 )  

                    ||(motion_kind == smash::hash40("special_lw1") && frame >25.0 )
                    ||(motion_kind == smash::hash40("special_air_lw1") && frame >25.0 )
                    ||(motion_kind == smash::hash40("special_lw2") && frame >17.0 )
                    ||(motion_kind == smash::hash40("special_air_lw2") && frame >17.0 )
                    ||(motion_kind == smash::hash40("special_lw3_end") && frame >31.0 )
                    ||(motion_kind == smash::hash40("special_lw3_end_air") && frame >8.0 )
                    ||(motion_kind == smash::hash40("special_air_lw3_end") && frame >31.0 )
                    ||(motion_kind == smash::hash40("special_lw3_end_air") && frame >8.0 ) 
                    ||(motion_kind == smash::hash40("throw_f") && frame >=13.0 )
                    ||(motion_kind == smash::hash40("throw_b") && frame >=16.0 )
                    ||(motion_kind == smash::hash40("throw_hi") && frame >=23.0 )
                    ||(motion_kind == smash::hash40("throw_lw") && frame >=16.0 )                   
                    || (motion_kind == smash::hash40("catch") && frame >7.0)
                    || (motion_kind == smash::hash40("catch_dash") && frame >10.0)
                    || (motion_kind == smash::hash40("catch_turn") && frame >11.0)
                ) {
                   CANCEL_IN_NEUTRAL[entry_id] = true;
                    if LIGHTNING[entry_id] {
                        if (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_JUMP || status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH || status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI3_END) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                   }
                }
        }
        else {
            CANCEL_IN_NEUTRAL [entry_id] = false;
        }
    }
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(once_per_fighter_frame);
}