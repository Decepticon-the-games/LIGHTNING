use super::*;
use smash::lib::L2CValue;
use crate::fighters::common::mechanics::cancels::motioncancels::WHIFF_CANCEL_INTO_SHIELD;

#[fighter_frame_callback]
pub fn shield_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        //REMOVE INVINCIBILITY ON SHIELD BREAK 
                
        if status_kind == *FIGHTER_STATUS_KIND_SHIELD_BREAK_FALL {
            macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
        }
        WorkModule::set_int(fighter.module_accessor, 100, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX); 
    }
}


//Hooking shield drop: 
//Param for shield drop lag. Whiff cancel into shield is punishable by extending the minimum time between being able to shield.
//Param for parry window is changed to 15
//We can also change the options out of shield if we wanna.

#[skyline::hook(replace = L2CFighterCommon_sub_guard_off_uniq)]
unsafe fn sub_guard_off_uniq(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if param_1.get_bool() {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME) > 0 {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME) == 0 {
                CancelModule::enable_cancel(fighter.module_accessor);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
            }
        }
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardOff_Common)]
unsafe fn status_guard_off_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    
    //Options out of shield
        let enables = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        ];
        for x in enables.iter() {
            WorkModule::enable_transition_term(fighter.module_accessor, *x);
        }
    //Parry window
    //let shield_just_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("shield_just_frame")) as f32; //Original
    let shield_just_frame = 15 as f32; //Override
    let just_shield_check_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("just_shield_check_frame"), 0);
    let just_frame = ((shield_just_frame * just_shield_check_frame) + 0.5) as i32;
    WorkModule::set_int(fighter.module_accessor, just_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);

    //Here we override the param "guard_off_cancel_frame" and set it to 1, for no shield drop lag.

        let guard_off_cancel_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("guard_off_cancel_frame")); //Original param
        //let guard_off_cancel_frame = 1; //Override
        WorkModule::set_int(fighter.module_accessor, guard_off_cancel_frame, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME); 
        let fighter_guard_off_cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("guard_off"), true);
        let ret_val = 1.0;
        let guard_off_work_cancel_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME); //Original
        //let guard_off_work_cancel_frame = 1; //Override

    //Here we override the param "guard_off_enable_shield_frame" (Minimum time between shields). If whiff cancel, it's set to 2 seconds, else it's 30 frames.

        //let guard_off_enable_shield_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("guard_off_enable_shield_frame")); //Original param. 
        if WHIFF_CANCEL_INTO_SHIELD[entry_id] {
            WorkModule::set_int(fighter.module_accessor, 120 + guard_off_work_cancel_frame, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_GUARD_FRAME); //120 Replaces guard_off_enable_shield_frame.
            WorkModule::set_int(fighter.module_accessor, 120 + guard_off_work_cancel_frame, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_ESCAPE_FRAME);
            WHIFF_CANCEL_INTO_SHIELD[entry_id] = false;
        }
        else {
            WorkModule::set_int(fighter.module_accessor, 30 + guard_off_work_cancel_frame, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_GUARD_FRAME); //30 Replaces guard_off_enable_shield_frame.
            WorkModule::set_int(fighter.module_accessor, 30 + guard_off_work_cancel_frame, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_ESCAPE_FRAME);
        }

    if !StopModule::is_stop(fighter.module_accessor) {
        sub_guard_off_uniq(fighter, false.into());
    }    
    fighter.global_table[0x15].assign(&L2CValue::Ptr(sub_guard_off_uniq as *const () as _)); 
    ret_val.into()
}
/*
#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardDamage_initStatus_Inner)]
unsafe fn sub_ftstatusuniqprocessguarddamage_initstatus_inner(fighter: &mut L2CFighterCommon) {
    let shield_power = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_POWER);
    let setoff_mul = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_SETOFF_MUL);
    let param_setoff_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_setoff_mul"));

    let mut shield_power = shield_power * setoff_mul * param_setoff_mul;
    let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_OBJECT_ID);
    
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        shield_power *= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("just_shield_setoff_mul"));
    }
    shield_power += WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_setoff_add"));
    let max = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x16a1c7df3f);
    if max < shield_power {
        shield_power = max;
    }

    if object_id != 0x50000000 {
        capture!(fighter, MA_MSC_CMD_CAPTURE_SET_IGNORE_OBJECT_ID, object_id);
        let mut cancel_frame = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
            shield_power
        } else {
            shield_power + WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("guard_off_cancel_frame")) as f32
        };
        cancel_frame *= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_ignore_capture_rate"));
        WorkModule::set_int(fighter.module_accessor, cancel_frame as i32, *FIGHTER_INSTANCE_WORK_ID_INT_GUARD_INVALID_CAPTURE_FRAME);
    } 
   
    WorkModule::set_int(fighter.module_accessor, shield_power as i32, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_STIFF_FRAME);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        let catch_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("shield_setoff_catch_frame"));
        if 0 < catch_frame {
            WorkModule::set_int(fighter.module_accessor, catch_frame, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CATCH_FRAME);
        }

        let motion_rate = smash::app::sv_fighter_util::get_guard_damage_motion_rate(fighter.lua_state_agent, Hash40::new("guard_damage"));
        let weight = MotionModule::weight(fighter.module_accessor);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("guard_damage"), 0.0, motion_rate, false, 0.0, false, false);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("guard"), 0.0, 1.0, false, 1.0);
            MotionModule::set_rate_2nd(fighter.module_accessor, 0.0);
            MotionModule::set_weight(fighter.module_accessor, weight, true);
            let prev_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_PREV_X);
            let prev_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_PREV_Y);
            fighter_status_guard::set_guard_blend_motion_angle(fighter, prev_x.into(), prev_y.into());
        }
    } else {
        let mut cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(0xfefe225e5), true);
        if cancel_frame == 0.0 {
            cancel_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new_raw(0xfefe225e5));
        }

        let adjusted = cancel_frame / shield_power.floor();
        if WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("just_shield_motion")) != 0 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0xfefe225e5), 0.0, adjusted, false, 0.0, false, false);
        } else {
            let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new_raw(0xfefe225e5));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0xfefe225e5), end_frame, adjusted, false, 0.0, false, false);
        }
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(0x1a29f56bfb), -1);
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_SHEILD_COUNT);
        if fighter_status_guard::is_continue_just_shield_count(fighter).get_bool() {
            ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, smash::app::ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
            ShieldModule::set_shield_type(fighter.module_accessor, smash::app::ShieldType(*SHIELD_TYPE_JUST_SHIELD), *FIGHTER_SHIELD_KIND_GUARD, 0);
            ReflectorModule::set_status(fighter.module_accessor, 0, smash::app::ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        } else {
            CancelModule::enable_cancel(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_DISABLE_HIT_STOP_DELAY_STICK);
        }
    } else {
        ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, smash::app::ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
        ControlModule::clear_command(fighter.module_accessor, false);
    }

    let mut setoff_speed = shield_power * WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_setoff_speed_mul"));
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        setoff_speed *= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("just_shield_speed_rate"));
    }

    setoff_speed = setoff_speed.min(WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_setoff_speed_max")));
    let shield_lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_LR);
    let directed_speed = setoff_speed * -shield_lr;
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE, ENERGY_STOP_RESET_TYPE_GUARD_DAMAGE, directed_speed, 0.0, 0.0, 0.0, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    let stop_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_HIT_STOP_FRAME);
    let stop_frame = stop_frame * WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x2434ca61df);
    WorkModule::set_int(fighter.module_accessor, stop_frame as i32, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_PREV_SHIELD_SCALE_FRAME);
    let hit_stop_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x20d241cd64);
    ShieldModule::set_hit_stop_mul(fighter.module_accessor, hit_stop_mul);
}
*/
fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_guard_off_uniq,
            status_guard_off_common,
        );
    }
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(shield_opff);
    skyline::nro::add_hook(nro_hook);
} 