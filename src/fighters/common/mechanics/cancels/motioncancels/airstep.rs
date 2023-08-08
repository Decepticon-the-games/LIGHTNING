#[fighter_frame_callback]
pub fn airdodge(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);  
        let air_move = ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 && ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_N)
        || ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 && ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_F)
        || ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 && ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_B)
        || ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 && ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI)
        || ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 && ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW)
        || ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0)
        || ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0)
        || ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0)
        || ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0);

        if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
           
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) && air_move {
                //Airstep
                if (frame >= 1.0 && frame <= 3.0) {

                    airstep(fighter);
                }       
            }
        }
    }
}

pub unsafe fn airdash(fighter : &mut L2CFighterCommon) {
    CancelModule::enable_cancel(fighter.module_accessor);
}

pub unsafe fn airstep(fighter : &mut L2CFighterCommon)  {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);  

    let dir_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
    let dir_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
    let stick_degrees = dir_y.atan2(dir_x).to_degrees();
    let zero = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};
    let rotation = smash::phx::Vector3f {x:0.0,y:0.0,z:stick_degrees};

    let nair = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 && ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_N;
    let fair = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 && ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_F;
    let bair = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 && ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_B;
    let upair = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 && ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI;
    let dair = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 && ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW;
    let neutral_b = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0;
    let side_b  = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0;
    let up_b = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0;
    let down_b = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0;

        if nair {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_n"), 0.0, 1.0, false, 0.0, false, false);
        }
        else if fair {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_f"), 0.0, 1.0, false, 0.0, false, false);
        }
        else if bair {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_b"), 0.0, 1.0, false, 0.0, false, false);
        }
        else if upair {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_hi"), 0.0, 1.0, false, 0.0, false, false);
        }
        else if dair {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_lw"), 0.0, 1.0, false, 0.0, false, false);
        }
        else if neutral_b {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
        }
        else if side_b {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
        }
        else if up_b {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
        }
        else if down_b {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
        }
        

        EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_sp_flash"), smash::phx::Hash40::new("waist"), &zero, &zero, 1.0, &zero, &zero, false, 0, 0, 0);//effect subject to change
        macros::BURN_COLOR(fighter, 0.0, 0.784, 1.0, 0.7);


}