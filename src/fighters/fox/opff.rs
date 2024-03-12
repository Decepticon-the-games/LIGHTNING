use super::*;


#[fighter_frame( agent = FIGHTER_KIND_FOX )]


    pub fn fox_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            let status_kind = StatusModule::status_kind(module_accessor);
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
            let max_jumps = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
            let jumps_used = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);

            //println!("fox-ill: {}", ILLUSION_CANCEL[entry_id]);

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

//jump cancel shine (multishine)
            if  (status_kind == *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP && frame >1.0 ) {
                if (max_jumps == 2 && jumps_used <2)
                {
                    if (ControlModule::is_enable_flick_jump(fighter.module_accessor) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
                        if situation_kind == *SITUATION_KIND_AIR {
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
                        }
                        if situation_kind == *SITUATION_KIND_GROUND {
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false);
                        }
                        else {
                            CancelModule::enable_cancel(fighter.module_accessor);
                        }
                    }
                }
            }
////multihit cancels


            //In Lightning...
            if IS_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING[entry_id] {
                //dair cancels after 3 successful hits, cancel into shine     
                let next_input = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0;
                multihit_counter(fighter, 0, 0, smash::hash40("attack_air_lw") , 3, next_input, 0, 0, smash::hash40("attack_air_lw") );
            }

        }
    }

pub fn install() {
    smashline::install_agent_frames!(
        fox_opff
    );

}