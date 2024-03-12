use super::*;
use crate::fighters::common::mechanics::cancels::motioncancels::WAVEDASH_MAGNET;

#[fighter_frame_callback]
pub fn wavedash_landing(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        
        if WAVEDASH[entry_id] {

            if status_kind == *FIGHTER_STATUS_KIND_LANDING || status_kind == *FIGHTER_STATUS_KIND_FALL {
                WAVEDASH[entry_id] = false;
            }
        }
    }
}

#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE, symbol = "_ZN7lua2cpp16L2CFighterCommon20status_pre_EscapeAirEv")]
pub unsafe fn status_pre_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue { //Borrowed code from HDR
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    use crate::fighters::common::mechanics::cancels::motioncancels::wavedash::handle_waveland;
    if WAVEDASH[entry_id] || handle_waveland(fighter, false) 
    {
        GroundModule::attach_ground(fighter.module_accessor, true);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into(); 
    }      
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_DISABLE,
        false,
        false,
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN, symbol = "_ZN7lua2cpp16L2CFighterCommon16status_EscapeAirEv")]
unsafe fn status_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue {

    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);  
    let frame = MotionModule::frame(fighter.module_accessor);

    let nair = ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_N;
    let fair = ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_F;
    let bair = ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_B;
    let upair = ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI;
    let dair = ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW;
    let neutral_b = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0;
    let side_b  = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0;
    let up_b = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0;
    let down_b = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0;


    fighter.sub_escape_air_common();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        if AIRSTEP[entry_id] {
            //if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) 
            //&& ! ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH)  {
                //AIRSTEP_EFFECT[entry_id] = true;
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
            //}
            /*else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                //special motions change by character. CREATE a variable that changes the motion kind based on each character!!!//
                AIRSTEP_EFFECT[entry_id] = true;
                if neutral_b {
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
            }*/
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air_slide"), 0.0, 1.0, false, 0.0, false, false); 
            }   
            //if ATTACK_CANCEL[entry_id] {
                AIRSTEP[entry_id] = false;
            //}
        }  
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air_slide"), 0.0, 1.0, false, 0.0, false, false);  
            WAVEDASH_MAGNET[entry_id] = true;
        }                                                                                                                                                                                                                                                                                                                                                                                                                                 
    } 
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air"), 0.0, 1.0, false, 0.0, false, false);
        WAVEDASH_MAGNET[entry_id] = false;
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(status_EscapeAir_Main as *const () as _))
}

unsafe extern "C" fn status_EscapeAir_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if WAVEDASH[entry_id] {
        return 1.into();
    }
    if !fighter.sub_escape_air_common_main().get_bool() {
        fighter.sub_escape_check_rumble();
    }
    0.into()
}

#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END, symbol = "_ZN7lua2cpp16L2CFighterCommon20status_end_EscapeAirEv")]
pub unsafe fn status_end_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if status_kind == FIGHTER_STATUS_KIND_FALL || status_kind == FIGHTER_STATUS_KIND_LANDING {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
            let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_frame_escape_air_slide_max"));
            WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            //let global_speed_mul = ParamModule::get_float(fighter.object(), ParamType::Common, "wavedash_speed_mul");
            //let global_speed_mul = 1.0;
            let speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("landing_speed_mul_escape_air_slide"));
            //let escape_air_slide_speed_clamp = WorkModule::get_param_float(fighter.module_accessor, hash40("escape_air_slide_speed"), 0) * global_speed_mul;
            let escape_air_slide_speed_clamp = 8.0;
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_x = (smash::app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent) * speed_mul).clamp(-escape_air_slide_speed_clamp, escape_air_slide_speed_clamp);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_y = (smash::app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent) * speed_mul).clamp(-escape_air_slide_speed_clamp, escape_air_slide_speed_clamp);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, speed_y);
            smash::app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        } else {
            let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("landing_frame_escape_air")) as f32;
            WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        }
        if status_kind == FIGHTER_STATUS_KIND_LANDING {
            //if AIRSTEP[entry_id] {
            //    AIRSTEP[entry_id] = false;
            //    fighter.change_status(FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR.into(), false.into());
            //}

            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_TURN);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP);
        }
    }
    WAVEDASH_MAGNET[entry_id] = true;
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_setup_escape_air_slide_common)]
unsafe fn setup_escape_air_slide_common(fighter: &mut L2CFighterCommon, stick_x: L2CValue, stick_y: L2CValue) {

    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let zero = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};

    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        return;
    }
    StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
    let stick_vec = sv_math::vec2_normalize(stick_x.get_f32(), stick_y.get_f32());
    WorkModule::set_float(fighter.module_accessor, stick_vec.x, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
    WorkModule::set_float(fighter.module_accessor, stick_vec.y, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
    //let escape_air_slide_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_speed")); //Original
    let escape_air_slide_speed = 8.0; //Override
    let escape_air_slide_speed_vec = Vector2f{x: escape_air_slide_speed * stick_vec.x, y: escape_air_slide_speed * stick_vec.y};
    
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_FREE, escape_air_slide_speed_vec.x, escape_air_slide_speed_vec.y, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, -1.0, -1.0);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);

    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fighter.module_accessor);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);

    let escape_air_slide_stiff_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_stiff_frame")); // new
    let escape_air_slide_u_stiff_frame = escape_air_slide_stiff_frame;
    let escape_air_slide_d_stiff_frame = escape_air_slide_stiff_frame;
    let dirx = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
    let diry = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
    let arctangent = diry.atan2(dirx.abs());
    let stiff_lerp = if 0.0 > arctangent.to_degrees() {
        fighter.lerp(
            escape_air_slide_stiff_frame.into(),
            escape_air_slide_d_stiff_frame.into(),
            (arctangent.to_degrees() / 90.0).into()
        )
    }
    else {
        fighter.lerp(
            escape_air_slide_stiff_frame.into(),
            escape_air_slide_u_stiff_frame.into(),
            (arctangent.to_degrees() / 90.0).into()
        )
    };
    let escape_air_slide_stiff_start_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_stiff_start_frame"));
    WorkModule::set_float(fighter.module_accessor, escape_air_slide_stiff_start_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_START_FRAME);
    //let escape_air_slide_back_end_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("escape_air_slide_back_end_frame"));
    let escape_air_slide_back_end_frame = 0;
    //let escape_air_add_xlu_start_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
    //WorkModule::set_int(fighter.module_accessor, escape_air_slide_back_end_frame + escape_air_add_xlu_start_frame, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_BACK_END_FRAME);
    WorkModule::set_int(fighter.module_accessor, escape_air_slide_back_end_frame, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_BACK_END_FRAME);
    WorkModule::set_float(fighter.module_accessor, escape_air_slide_stiff_frame, *FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
   
    //WorkModule::set_int(fighter.module_accessor, 30, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_STEP);
    //WorkModule::set_int(fighter.module_accessor, 30, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
    
    let dir_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
    let dir_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
    let stick_degrees = dir_y.atan2(dir_x).to_degrees();
    let zero = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};
    let rotation = smash::phx::Vector3f {x:0.0,y:0.0,z:stick_degrees};

    if AIRSTEP[entry_id] {
        EffectModule::req_on_joint(
            fighter.module_accessor, 
            smash::phx::Hash40::new("sys_sp_flash"), 
            smash::phx::Hash40::new("waist"), 
            &zero, 
            &zero, 
            0.5, 
            &zero, 
            &zero, 
            false, 
            0, 
            0, 
            0
        );
        //effect subject to change
        macros::LAST_EFFECT_SET_COLOR(fighter,0.0, 0.784, 1.0,);
    } 
    else if (IS_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING[entry_id] && WAVEDASH[entry_id] == false) || CANCEL_IN_NEUTRAL[entry_id] {
        EffectModule::req_on_joint(
            fighter.module_accessor, 
            smash::phx::Hash40::new("sys_attack_speedline"), 
            smash::phx::Hash40::new("waist"), 
            &zero, 
            &rotation, 
            1.0, 
            &zero, 
            &zero, 
            false,
            0, 
            0, 
            0
        );              
        macros::LAST_EFFECT_SET_COLOR(fighter,0.0, 0.784, 1.0,);
    }
    else if WAVEDASH[entry_id] == false {
        EffectModule::req_on_joint(
            fighter.module_accessor, 
            smash::phx::Hash40::new("sys_attack_speedline"), 
            smash::phx::Hash40::new("waist"), 
            &zero, &rotation, 
            1.0, 
            &zero, 
            &zero, 
            false, 
            0, 
            0, 
            0
        );              
    }
    /*
    else {
        EffectModule::req_on_joint(
            fighter.module_accessor,
            Hash40::new("sys_smash_flash_s"),
            Hash40::new("hip"),
            &Vector3f{x: 0.0, y: 4.0, z: 8.0},
            &zero,
            1.0,
            &Vector3f{x: 18.0, y: 12.0, z: 0.0},
            &zero,
            false,
            0,
            0,
            0
        );    
    }
    */
}


fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            setup_escape_air_slide_common
        );
    }
}
pub fn install() {
    install_status_scripts!(
        status_pre_EscapeAir,
        status_EscapeAir,
        //status_end_EscapeAir
    );
    skyline::nro::add_hook(nro_hook);
    smashline::install_agent_frame_callbacks!(wavedash_landing);
}