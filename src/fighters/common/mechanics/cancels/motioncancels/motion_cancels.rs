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
        && ! (AttackModule::is_attack_occur(fighter.module_accessor) && status_kind != *FIGHTER_STATUS_KIND_THROW)
        //&& ! CatchModule::is_catch(fighter.module_accessor) 
        {

            //
            if fighter_kind == *FIGHTER_KIND_PICKEL
                && (
                    (motion_kind == smash::hash40("attack_s3") && frame > 6.0 )
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
        }
    }
}


pub fn install() {
smashline::install_agent_frame_callbacks!(motion_cancels);
}