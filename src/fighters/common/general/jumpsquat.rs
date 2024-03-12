//HOOK JUMPSQUAT TO BUFFER DIRECTIONAL AIRDODGE WHEN PRESSING AIRDODGE
use super::*;

#[skyline::hook(replace = L2CFighterCommon_uniq_process_JumpSquat_exec_status_param)]
unsafe fn uniq_process_jumpsquat_exec_status_param(fighter: &mut L2CFighterCommon, param_1: L2CValue) {

    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 0);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor); 
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);  
    let end_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("fighter_param"), hash40("jump_squat_frame")) as f32;

    if !param_1.get_bool() {
        fighter.sub_jump_squat_uniq_check_sub(FIGHTER_STATUS_JUMP_FLAG_BUTTON.into());
        fighter.sub_jump_squat_uniq_check_sub_mini_attack();
    }
    else {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(param_1.get_ptr());
        callable(fighter);
    }

    if //frame >= end_frame
    MotionModule::is_end(fighter.module_accessor) 
     {
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
            let stick_degrees = ControlModule::get_stick_angle(fighter.module_accessor).to_degrees();
            if stick_degrees > -180.0 && stick_degrees < -0.0
            {
                WAVEDASH[entry_id] = true;
                GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_ESCAPE);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
            }
            else {
                WAVEDASH[entry_id] = false;
                GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
            }
        }
        else {
            WAVEDASH[entry_id] = false;
            GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FROM_SQUAT, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_JUMP_FROM);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START);
        } 
    }
}


fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            uniq_process_jumpsquat_exec_status_param,
        );
    }
}
pub fn install() {
    skyline::nro::add_hook(nro_hook);
}