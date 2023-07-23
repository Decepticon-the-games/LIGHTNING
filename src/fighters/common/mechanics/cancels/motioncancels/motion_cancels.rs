use super::*;

#[fighter_frame_callback]
pub fn motion_cancels(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);

        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        //let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 0);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
        let frame = MotionModule::frame(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        
        if CANCEL_IN_NEUTRAL [entry_id] == false 
        && ! DISABLE_CANCEL_IN_NEUTRAL[entry_id]
        && ! (AttackModule::is_attack_occur(fighter.module_accessor) && status_kind != *FIGHTER_STATUS_KIND_THROW)
        //&& ! CatchModule::is_catch(fighter.module_accessor) 
        {

            //

            if fighter_kind == *FIGHTER_KIND_INKLING 
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 10.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >13.0 )
                    ||(status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 && frame >17.0 )
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
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                }
                } 
            if fighter_kind == *FIGHTER_KIND_RIDLEY 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 11.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >13.0 )
                    ||(status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 && frame >21.0 )
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
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                }
                } 
            if (fighter_kind == *FIGHTER_KIND_SIMON || fighter_kind == *FIGHTER_KIND_RICHTER)
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 13.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >22.0 )
                    ||(motion_kind == smash::hash40("attack_lw3_2") && frame >28.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >24.0 )
                    ||((status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 ) && frame >25.0 )
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
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                }
                } 
            if fighter_kind == *FIGHTER_KIND_KROOL
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 13.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >15.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >25.0 )
                    ||(status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 && frame >20.0 )
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
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                }
                } 
            if fighter_kind == *FIGHTER_KIND_SHIZUE
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 11.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >13.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >10.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >19.0 )
                    ||(status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 && frame >19.0 )
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
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                }
                } 
            if fighter_kind == *FIGHTER_KIND_GAOGAEN 
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 14.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >15.0 )
                    ||(status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 && frame >18.0 )
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
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
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
                    ||(status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 && frame >17.0 )
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
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
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
                    || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 && frame >19.0 )
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
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
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
                    || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 && frame >19.0 )
                    || (motion_kind == smash::hash40("attack_hi4") && frame >17.0 )
                    || (motion_kind == smash::hash40("attack_lw4") && frame >21.0 )
                    || (motion_kind == smash::hash40("attack_air_n") && frame >16.0 )
                    || (motion_kind == smash::hash40("attack_air_f") && frame >17.0 )
                    || (motion_kind == smash::hash40("attack_air_b") && frame >20.0 )
                    || (motion_kind == smash::hash40("attack_air_hi") && frame >10.0 )
                    || (motion_kind == smash::hash40("attack_air_lw") && frame >24.0 )
                    || (motion_kind == smash::hash40("special_n1") && frame >10.0 )
                    || (motion_kind == smash::hash40("special_air_n1") && frame >10.0 )
                    || (motion_kind == smash::hash40("special_n2") && frame >11.0 )
                    || (motion_kind == smash::hash40("special_air_n2") && frame >11.0 )
                    || (motion_kind == smash::hash40("special_n3") && frame >16.0 )
                    || (motion_kind == smash::hash40("special_air_n3") && frame >16.0 )
                    || (motion_kind == smash::hash40("special_s1") && frame >9.0 )
                    || (motion_kind == smash::hash40("special_air_s1") && frame >9.0 )
                    || (motion_kind == smash::hash40("special_s2") && frame >10.0 )
                    || (motion_kind == smash::hash40("special_air_s2") && frame >10.0 )
                    || (motion_kind == smash::hash40("special_s3") && frame >43.0 )
                    || (motion_kind == smash::hash40("special_air_s3") && frame >43.0 )
                    || (motion_kind == smash::hash40("special_hi1") && frame > 30.0 )
                    || (motion_kind == smash::hash40("special_air_hi1") && frame >30.0 )
                    || (motion_kind == smash::hash40("special_hi2") && frame > 40.0 )
                    || (motion_kind == smash::hash40("special_air_hi2") && frame >40.0 )
                    || (motion_kind == smash::hash40("special_hi3") && frame > 60.0 )
                    || (motion_kind == smash::hash40("special_air_hi3") && frame >60.0 )
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
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                }
                }
            if fighter_kind == *FIGHTER_KIND_BUDDY
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 9.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame >14.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame >21.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame >20.0 )
                    || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 && frame >21.0 )
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
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                }
                }
            if fighter_kind == *FIGHTER_KIND_DOLLY
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 13.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame >11.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame >9.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame >22.0 )
                    || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 && frame >21.0 )
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
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                }
                    
                }
            if fighter_kind == *FIGHTER_KIND_MASTER
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 10.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame >15.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame >15.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame >11.0 )
                    || ((status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4) && frame >25.0 )
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
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                }
                }
            if fighter_kind == *FIGHTER_KIND_TANTAN
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame == 0.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame >15.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame >19.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame >16.0 )
                    || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 && frame == 0.0 )
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
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                }
                }
            if fighter_kind == *FIGHTER_KIND_PICKEL
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 6.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame >9.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame >12.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame >13.0 )
                    || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 && frame >15.0 )
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
                    || (motion_kind == smash::hash40("special_air_hi_max") && frame >1.0 )
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
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                }
                }
            if fighter_kind == *FIGHTER_KIND_EDGE
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 16.0 )
                    || (motion_kind == smash::hash40("attack_hi3") && frame >25.0 )
                    || (motion_kind == smash::hash40("attack_lw3") && frame >22.0 )
                    || (motion_kind == smash::hash40("attack_dash") && frame >19.0 )
                    || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 && frame >25.0 )
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
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                }
                }
            if fighter_kind == *FIGHTER_KIND_EFLAME
                && (
                    (motion_kind == smash::hash40("attack_s3_s") && frame > 13.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >16.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >11.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >18.0 )
                    ||(status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 && frame >23.0 )
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
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                }
                }
            if fighter_kind == *FIGHTER_KIND_ELIGHT
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 9.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >7.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >10.0 )
                    ||(status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 && frame >17.0 )
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
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
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
                ||(status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 && frame >26.0 )
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
                                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
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
                    ||(status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 && frame >20.0 )
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
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                }
                }

            if fighter_kind == *FIGHTER_KIND_MIISWORDSMAN
                && (
                    ((motion_kind == smash::hash40("attack_s3_s") || motion_kind == smash::hash40("attack_s3_hi") || motion_kind == smash::hash40("attack_s3_lw")) && frame > 11.0 )
                    ||(motion_kind == smash::hash40("attack_hi3") && frame >12.0 )
                    ||(motion_kind == smash::hash40("attack_lw3") && frame >6.0 )
                    ||(motion_kind == smash::hash40("attack_dash") && frame >10.0 )
                    ||((status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 || motion_kind == smash::hash40("attack_s4_hi") ||motion_kind == smash::hash40("attack_s4_lw")) && frame >17.0 )
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
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        }
                }
                }
        }
        else {
            //CANCEL_IN_NEUTRAL [entry_id] = false;
        }
    }
}


pub fn install() {
smashline::install_agent_frame_callbacks!(motion_cancels);
}