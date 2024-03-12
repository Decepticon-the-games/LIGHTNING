use super::*;


pub static mut FIGHTER_STATUS_KIND_CROSS_CANCEL : [bool; 8] = [false; 8];
pub static mut FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CROSS_CANCEL : [bool; 8] = [false; 8];
pub static mut CROSS_CANCEL_SETUP : [bool; 8] = [false; 8];
static mut CAMERA : [bool; 8] = [false; 8];
pub static mut OPPONENT_X : [f32; 8] = [0.0; 8];
pub static mut OPPONENT_Y : [f32; 8] = [0.0; 8];
pub static mut OPPONENT_BOMA  : [u64; 8] = [0; 8];
static mut YOU_X : [f32; 8] = [0.0; 8];
static mut YOU_Y : [f32; 8] = [0.0; 8];
static mut SEC_SEN_TIMER : [f32; 8] = [-0.2; 8]; // I start this as -0.4 so that Ryu doesn't immediately start dodging, there's a little pause before he does
static mut OPPONENT_DIRECTION : [f32; 8] = [12.0; 8];
static mut VERT_EXTRA : [f32; 8] = [12.0; 8];
static mut SEC_SEN_DIREC : [i32; 8] = [0; 8];
static mut FLASH_TIMER : [i16; 8] = [-1; 8];

//ULTRA INSTINCT

#[fighter_frame_callback]
pub fn cross_cancel_condition(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let up_taunt_motion = (motion_kind == smash::hash40("appeal_hi_l")||motion_kind == smash::hash40("appeal_hi_r"));

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {

            //cross_cancel_setup(fighter);//testing only

            if up_taunt_motion
            {
                FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CROSS_CANCEL[entry_id] = true;
            }
            else {
                FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CROSS_CANCEL[entry_id] = false;
            }

            if FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CROSS_CANCEL[entry_id] 
            //&& CancelModule::is_enable_cancel(fighter.module_accessor) 
            {
                //StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                cross_cancel_setup(fighter);
                FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CROSS_CANCEL[entry_id] = false;
            }
        }
    }
}
pub fn cross_cancel_setup(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        if FIGHTER_STATUS_KIND_CROSS_CANCEL [entry_id] == false
        {
            //StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
            if MotionModule::frame(fighter.module_accessor) == 4.0 {
                EffectModule::req_emit(fighter.module_accessor, Hash40::new("sys_aura_light"), 0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
                macros::PLAY_SE(fighter, Hash40::new("se_ryu_6c_aura"));
                
                FLASH_TIMER[entry_id] = -1;
            }
            if MotionModule::frame(fighter.module_accessor) <= 30.0
            && MotionModule::frame(fighter.module_accessor) >= 4.0 {
                //CAMERA[entry_id] = false;
                CROSS_CANCEL_SETUP[entry_id] = true;
                DamageModule::set_damage_lock(fighter.module_accessor, true);
                DamageModule::set_no_reaction_no_effect(fighter.module_accessor, true);
                HitModule::set_hit_stop_mul(fighter.module_accessor, 0.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
            }
            else {
                DamageModule::set_damage_lock(fighter.module_accessor, false);
                DamageModule::set_no_reaction_no_effect(fighter.module_accessor, false);
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), true, true);
                HitModule::set_hit_stop_mul(fighter.module_accessor, 1.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
                macros::COL_NORMAL(fighter);
                CROSS_CANCEL_SETUP[entry_id] = false;
                //macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);  
            }
            /*if MotionModule::frame(fighter.module_accessor) > 30.0 {
                CROSS_CANCEL_SETUP[entry_id] = false;
            }*/
        }
        else if FIGHTER_STATUS_KIND_CROSS_CANCEL[entry_id] == false // Turns off all of the effects of Secret Sensation.
        && CROSS_CANCEL_SETUP[entry_id] {
            DamageModule::set_damage_lock(fighter.module_accessor, false);
            DamageModule::set_no_reaction_no_effect(fighter.module_accessor, false);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), true, true);
            HitModule::set_hit_stop_mul(fighter.module_accessor, 1.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
            macros::COL_NORMAL(fighter);
            CROSS_CANCEL_SETUP[entry_id] = false;
            //macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
        }          

        // Handles the blue flashes on Ryu during the counter state

        if CROSS_CANCEL_SETUP[entry_id] {

            if FLASH_TIMER[entry_id] < 0 {
                FLASH_TIMER[entry_id] = 8;
            }
            if FLASH_TIMER[entry_id] <= 4 {
                macros::COL_NORMAL(fighter);
                FLASH_TIMER[entry_id] -= 1;
            }
            if FLASH_TIMER[entry_id] > 4 {
                macros::FLASH(fighter, 0, 0.55, 1, 1.75);
                FLASH_TIMER[entry_id] -= 1;
            }
        }
    }
}
pub fn enable_cross_cancel_effects(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let opponent_boma = sv_battle_object::module_accessor(WHO_GOT_HIT_BOMA[entry_id]);
        let zero = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};
        let rotation = smash::phx::Vector3f {x:0.0,y:0.0,z:90.0}; 

        EffectModule::req_emit(fighter.module_accessor, Hash40::new("sys_aura_light"), 0);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
        DamageModule::set_damage_lock(fighter.module_accessor, true); 
        DamageModule::set_no_reaction_no_effect(fighter.module_accessor, true); 
        HitModule::set_hit_stop_mul(fighter.module_accessor, 0.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0); 
        //macros::PLAY_SE(fighter, Hash40::new("se_ryu_6c_exec"));
        macros::CAM_ZOOM_IN_arg5(fighter, 3.0, 0.0, 1.5, 0.0, 0.0); 
        macros::SLOW_OPPONENT(fighter, 999.0, 1.0); 
        SlowModule::set_whole(fighter.module_accessor, 4, 32); 
        macros::FILL_SCREEN_MODEL_COLOR(fighter, 0, 12, 0.1, 0.1, 0.1, 0, 0.001, 0.011, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);                    
             
    }
}
pub fn disable_cross_cancel_effects(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let opponent_boma = sv_battle_object::module_accessor(WHO_GOT_HIT_BOMA[entry_id]);
        let zero = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};
        let rotation = smash::phx::Vector3f {x:0.0,y:0.0,z:90.0};
        
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE); 
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK); 
        macros::SET_SPEED_EX(fighter, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            PostureModule::reverse_lr(fighter.module_accessor); 
        }
        macros::CAM_ZOOM_OUT(fighter); 
        macros::CANCEL_FILL_SCREEN(fighter, 0, 5); 
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), true, true);
        SlowModule::clear_whole(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        JostleModule::set_status(fighter.module_accessor, true);
    }
}
#[fighter_frame_callback]
pub fn ultrainstinct(fighter : &mut L2CFighterCommon) {
    unsafe {
        //let fighter.module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        if entry_id < 8 {

            // Reset Vars

            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
                FIGHTER_STATUS_KIND_CROSS_CANCEL[entry_id] = false;
                CROSS_CANCEL_SETUP[entry_id] = false;
            }

            if FIGHTER_STATUS_KIND_CROSS_CANCEL[entry_id] {
                
            
                if CAMERA[entry_id] == false {
                    enable_cross_cancel_effects(fighter);
                    YOU_X[entry_id] = PostureModule::pos_x(fighter.module_accessor);
                    YOU_Y[entry_id] = PostureModule::pos_y(fighter.module_accessor);
                    if YOU_X[entry_id] == OPPONENT_X[entry_id] { 
                        OPPONENT_DIRECTION[entry_id] = -12.0 * PostureModule::lr(fighter.module_accessor);
                        if fighter_kind == *FIGHTER_KIND_RYU
                        || fighter_kind == *FIGHTER_KIND_KEN {
                            SEC_SEN_DIREC[entry_id] = *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B;
                        }
                        else {
                            SEC_SEN_DIREC[entry_id] = *FIGHTER_STATUS_KIND_ESCAPE;
                        }
                    }
                    else if YOU_X[entry_id] < OPPONENT_X[entry_id] {
                        OPPONENT_DIRECTION[entry_id] = 12.0;
                        if PostureModule::lr(fighter.module_accessor) == -1.0 {
                            PostureModule::reverse_lr(fighter.module_accessor);
                        }
                        if fighter_kind == *FIGHTER_KIND_RYU
                        || fighter_kind == *FIGHTER_KIND_KEN {
                            SEC_SEN_DIREC[entry_id] = *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F;
                        }
                        else {
                            SEC_SEN_DIREC[entry_id] = *FIGHTER_STATUS_KIND_ESCAPE;
                        }
                    }
                    else {
                        OPPONENT_DIRECTION[entry_id] = -12.0;
                        if PostureModule::lr(fighter.module_accessor) == 1.0 {
                            PostureModule::reverse_lr(fighter.module_accessor);
                        }
                        if fighter_kind == *FIGHTER_KIND_RYU
                        || fighter_kind == *FIGHTER_KIND_KEN {
                            SEC_SEN_DIREC[entry_id] = *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F;
                        }
                        else {
                            SEC_SEN_DIREC[entry_id] = *FIGHTER_STATUS_KIND_ESCAPE;
                        }
                    }
                    if (YOU_Y[entry_id] - OPPONENT_Y[entry_id]).abs() <= 12.0
                    && StatusModule::situation_kind(OPPONENT_BOMA [entry_id] as *mut smash::app::BattleObjectModuleAccessor) == *SITUATION_KIND_GROUND {
                        VERT_EXTRA[entry_id] = 0.0; 
                    }
                    else {
                        StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), true); 
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE); 
                        VERT_EXTRA[entry_id] = 12.0; 
                        YOU_Y[entry_id] += 2.0; 
                        PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                            x: 0.0,
                            y: 2.0
                        });
                    }
                    CAMERA[entry_id] = true; 
                    CROSS_CANCEL_SETUP[entry_id] = false;
                }

                if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                    if SEC_SEN_DIREC[entry_id] == *FIGHTER_STATUS_KIND_ESCAPE {
                        SEC_SEN_DIREC[entry_id] = *FIGHTER_STATUS_KIND_ESCAPE_AIR;
                    }
                }
                
                if (YOU_Y[entry_id] - OPPONENT_Y[entry_id]).abs() <= 12.0 
                && StatusModule::situation_kind(OPPONENT_BOMA [entry_id] as *mut smash::app::BattleObjectModuleAccessor) == *SITUATION_KIND_GROUND {
                    GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                }
                if StatusModule::status_kind(fighter.module_accessor) != SEC_SEN_DIREC[entry_id] { 
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
                    StatusModule::change_status_request_from_script(fighter.module_accessor, SEC_SEN_DIREC[entry_id], true);
                }
                if SEC_SEN_TIMER[entry_id] >= 0.0 { 
                    if (YOU_Y[entry_id] - OPPONENT_Y[entry_id]).abs() > 12.0 { 
                        StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), true); 
                    }
                    PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f{ 
                        x: (((OPPONENT_X[entry_id] +OPPONENT_DIRECTION[entry_id]) * SEC_SEN_TIMER[entry_id]) + YOU_X[entry_id] * (1.0 - SEC_SEN_TIMER[entry_id])),
                        y: (((OPPONENT_Y[entry_id] + VERT_EXTRA[entry_id]) * SEC_SEN_TIMER[entry_id]) + YOU_Y[entry_id] * (1.0 - SEC_SEN_TIMER[entry_id])) 
                    });
                }
                SEC_SEN_TIMER[entry_id] += 0.1; 
                if SEC_SEN_TIMER[entry_id] > 1.0 {
                    FIGHTER_STATUS_KIND_CROSS_CANCEL[entry_id] = false;
                    CAMERA[entry_id] = false;
                    disable_cross_cancel_effects(fighter);
                    SEC_SEN_TIMER[entry_id] = -0.2; 
                }
            }
        }
    }
}


pub fn install() {

   
    smashline::install_agent_frame_callbacks!(ultrainstinct, cross_cancel_condition);
}