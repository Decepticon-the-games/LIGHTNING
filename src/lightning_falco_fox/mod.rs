use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;






#[fighter_frame( agent = FIGHTER_KIND_FALCO )]
pub fn once_per_fighter_frame_falco(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        if (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 && frame > 12.0) || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 && frame > 12.0) {
                     if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor) {
                        CancelModule::enable_cancel(fighter.module_accessor);
                    }   
}


        if (MotionModule::motion_kind(module_accessor) == smash::hash40("special_n_loop")
        || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_n_loop"))
        && (ControlModule::get_command_flag_cat(module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 {
            WorkModule::set_flag(module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        }

        if MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw")
        ||MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw_r") 
        || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw")
        || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw_r") {
            if MotionModule::frame(module_accessor) >= 7.0 {
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0  {
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
            }
        }
        //Shine cancel Jump (multishine)
        if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT 
        && MotionModule::frame(module_accessor) > 1.0 {
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
        if MotionModule::motion_kind(module_accessor) == smash::hash40("special_s") {
            if AttackModule:: is_attack_occur(fighter.module_accessor) {
                if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
                || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
            }
        }
        
        else if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK) 
        && ! (status_kind == *FIGHTER_STATUS_KIND_FINAL)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
        //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3)
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
                     if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor) {
                        CancelModule::enable_cancel(fighter.module_accessor);
                    }   
}
    }                                      
}

#[fighter_frame( agent = FIGHTER_KIND_FOX )]
pub fn once_per_fighter_frame_fox(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        //Fast fall laser
        if (MotionModule::motion_kind(module_accessor) == smash::hash40("special_n_loop")
        || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_n_loop"))
        && (ControlModule::get_command_flag_cat(module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 {
            WorkModule::set_flag(module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        }
        //Jump cancel shine    
        if MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw_loop")
        || MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw_loop_l")
        || MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw_hit")
        || MotionModule::motion_kind(module_accessor) == smash::hash40("special_lw_hit_l") 
        || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw_loop")
        || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw_loop_l")
        || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw_hit")
        || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_lw_hit_l") {
            if MotionModule::frame(module_accessor) >= 1.0 {
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
                    //CancelModule::enable_cancel(fighter.module_accessor);
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }
        //Shine cancel Jump (multishine)
        if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT 
        && MotionModule::frame(module_accessor) > 1.0 {
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
            
        else if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK) 
        && ! (status_kind == *FIGHTER_STATUS_KIND_FINAL)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
        //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3)
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
                     if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor) {
                        CancelModule::enable_cancel(fighter.module_accessor);
                    }   
}
    }                                      
}

pub fn install() {
    smashline::install_agent_frames!(once_per_fighter_frame_fox, once_per_fighter_frame_falco);
}