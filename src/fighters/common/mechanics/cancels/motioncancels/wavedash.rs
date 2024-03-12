use super::*;

//EASIER WAVEDASH CHAINS// 
#[fighter_frame_callback]
pub fn wavedash_chains(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);        
        let frame = MotionModule::frame(fighter.module_accessor);
        
        if status_kind == *FIGHTER_STATUS_KIND_LANDING {
            if frame >10.0 
            //&& ((ControlModule::is_enable_flick_jump(fighter.module_accessor) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0) 
            {
                CancelModule::enable_cancel(fighter.module_accessor);
            }  
            else if frame <10.0 && frame >1.0 
            && (ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL))
            {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }  
    }
}
//Borrowed form HDR
pub unsafe fn handle_waveland(fighter : &mut L2CFighterCommon, require_airdodge: bool) -> bool {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 0);

    // MotionModule::frame(fighter.module_accessor) > 5.0 && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_FLAG_HIT_XLU);
    if (require_airdodge && ![*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind))
    || KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_FALL {
        return false;
    }

    // must check this because it is for allowing the player to screw up a perfect WD and be punished with a non-perfect WD (otherwise they'd have like, 8 frames for perfect WD lol)
    if WAVEDASH_MAGNET[entry_id] {
        return false;
    }

    if prev_status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
        return false;
    }

    // The distance from your ECB's bottom point to your Top bone is your waveland snap threshold
    let ecb_bottom = *GroundModule::get_rhombus(fighter.module_accessor, true).add(1);
    let pos = *PostureModule::pos(fighter.module_accessor);
    let snap_leniency = if WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y) <= 0.0 {
            // For a downwards/horizontal airdodge, waveland snap threshold = the distance from your ECB bottom to your Top bone
            ecb_bottom.y - pos.y
        } else {
            // For an upwards airdodge, waveland snap threshold = 5 units below ECB bottom, if the distance from your ECB bottom to your Top bone is < 5
            (ecb_bottom.y - pos.y).max(6.0)
        };
    let zero = Vector2f{x:0.0, y:0.0};
    let line_bottom = Vector2f{x:ecb_bottom.x, y:ecb_bottom.y - snap_leniency};
    let mut ground_pos_any = zero;
    let mut ground_pos_stage = zero;
    GroundModule::line_segment_check(fighter.module_accessor, &Vector2f{x:ecb_bottom.x, y:ecb_bottom.y}, &line_bottom, &zero, &mut ground_pos_any, true);
    GroundModule::line_segment_check(fighter.module_accessor, &Vector2f{x:ecb_bottom.x, y:ecb_bottom.y}, &line_bottom, &zero, &mut ground_pos_stage, false);
    let can_snap = ground_pos_any != zero && (ground_pos_stage == zero
        || WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y) <= 0.0);
    if can_snap { // pretty sure it returns a pointer, at least it defo returns a non-0 value if success
        //crate::VarModule::on_flag(fighter.module_accessor.object(), crate::consts::vars::common::status::DISABLE_ECB_SHIFT);
        PostureModule::set_pos(fighter.module_accessor, &Vector3f{x:pos.x, y:ground_pos_any.y + 0.1, z:pos.z});
        GroundModule::attach_ground(fighter.module_accessor, false);
        true
    } else {
        false
    }
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(wavedash_chains);
}