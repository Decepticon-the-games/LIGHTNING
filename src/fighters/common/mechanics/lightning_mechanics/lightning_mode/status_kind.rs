use super::*;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING: i32 = 0x0100;
pub const FIGHTER_INSTANCE_WORK_ID_INT_LIGHTNING_TIMER: i32 = FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING+1;
//pub const FIGHTER_STATUS_KIND_LIGHTNING: LuaConst = 0x153f;// find appropriate int

//#[common_status_script(status = FIGHTER_STATUS_KIND_LIGHTNING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub fn status_lightning_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        //MotionModule::change_motion(fighter.module_accessor, Hash40::new("_______"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING);
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING) {
            lightning_effects(fighter);
            WorkModule::set_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_LIGHTNING_TIMER, 3600);//sets the timer to a conditional value.
        }
    }
    0.into()
}
/*#[common_status_script(status = FIGHTER_STATUS_KIND_LIGHTNING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub fn status_lightning(fighter: &mut L2CFighterCommon) -> L2CValue {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        //MotionModule::change_motion(fighter.module_accessor, Hash40::new("_______"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING);
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING) {

        }
    }
    0.into()
}*/

/*#[common_status_script(status = FIGHTER_STATUS_KIND_LIGHTNING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_EXEC)]
pub fn status_lightning_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    unsafe {

    }
}*/

/*#[common_status_script(status = FIGHTER_STATUS_KIND_LIGHTNING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub fn status_lightning_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    unsafe {

    }
}*/

#[fighter_frame_callback]
pub fn lightning_button(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        let hitlag = (SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) > 0 || StopModule::is_stop(fighter.module_accessor));
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let idles =  (status_kind == *FIGHTER_STATUS_KIND_WAIT || status_kind == *FIGHTER_STATUS_KIND_JUMP || status_kind == *FIGHTER_STATUS_KIND_FALL);

        println!("lightning: {}", WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_LIGHTNING_TIMER));
        //if attack cancel and the meter is filled, can_ligtning is enabled. Burn color to lightning blue.

        if lightning_mode_conditions(fighter)
        {
            if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0 
            && (idles || is_after_hitlag(fighter))
            {
                LIGHTNING_BUTTON[entry_id] = true;

            }
            else {
                LIGHTNING_BUTTON[entry_id] = false;
            }
            ///cpu testing only////
            if smash::app::sv_information::is_ready_go() == true {
                //LIGHTNING_BUTTON[entry_id] = true;
            }
            //////////////////////
            if LIGHTNING_BUTTON[entry_id] 
            {
                //IS_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING[entry_id] = true;
                //StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LIGHTNING, false);
                status_lightning_pre(fighter);
                LIGHTNING_BUTTON[entry_id] = false;
            }            
        }
    }
}
pub unsafe fn lightning_mode_conditions(fighter : &mut L2CFighterCommon) -> bool {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        LIGHTNING_TIMER[entry_id] == -1
        && CAN_LIGHTNING[entry_id]
    }
}
#[fighter_frame_callback]
pub fn lightning_status(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING) {
            macros::BURN_COLOR(fighter, 0.0, 0.784, 1.0, 0.7);

            //every attack produces an effect
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
                let pos = smash::phx::Vector3f {x:AttackModule::pos_x_2(fighter.module_accessor),y:AttackModule::pos_y(fighter.module_accessor),z:0.0};
                let zero = smash::phx::Vector3f {x:0.0 ,y:0.0,z:0.0};
                EffectModule::req(fighter.module_accessor, smash::phx::Hash40::new("sys_hit_elec"), &pos, &zero, 0.5, 0, 0, false, 0);    
                macros::LAST_EFFECT_SET_COLOR(fighter,0.0, 0.784, 1.0,);          
            } 
            //Countdown
            if WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_LIGHTNING_TIMER) > 0 {
                if ! (/*status_kind == FIGHTER_STATUS_KIND_LIGHTNING ||*/ status_kind == *FIGHTER_STATUS_KIND_REBIRTH || status_kind == *FIGHTER_STATUS_KIND_DEAD) {//Will halt countdown on death/respawn
                    //WorkModule::dec_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_LIGHTNING_TIMER);
                }
            }
            //STOP LIGHTNING
            if WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_LIGHTNING_TIMER) == 0 { // Only when the timer reaches 0 will u be unable to perform lightning again for the rest of the match
                //CAN_LIGHTNING = CAN_LIGHTNING_TEMP;
                //lightning_disable(fighter);
                //WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING);
            }     
            
        }   
    }
}
//VISUAL EFFECTS
pub unsafe fn lightning_effects(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    EffectModule::req_emit(fighter.module_accessor, Hash40::new("sys_aura_light"), 0);
    macros::LAST_EFFECT_SET_COLOR(fighter,0.0, 0.784, 1.0,);
    //EffectModule::set_rgb(fighter.module_accessor, 0,0.0, 0.784, 1.0);
    EffectModule::req_emit(fighter.module_accessor, Hash40::new("sys_aura_light"), 0); 
    macros::LAST_EFFECT_SET_COLOR(fighter,0.0, 0.784, 1.0,); 
    //ffectModule::set_rgb(fighter.module_accessor, 0,0.0, 0.784, 1.0);
    ModelModule::enable_gold_eye(fighter.module_accessor);	 
    //ModelModule::set_color_rgb(fighter.module_accessor, 0.0, 0.784, 1.0, MODEL_COLOR_TYPE::MODEL_COLOR_TYPE_NORMAL);
    WorkModule::set_float(fighter.module_accessor, 50.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHARGE_FINAL_ADD_DAMAGE)
    /*if ATTACK_CANCEL[entry_id] {

    }*/
}
//DISABLE
pub unsafe fn lightning_disable(fighter : &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    LIGHTNING_TIMER[entry_id] = -1;
    IS_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING[entry_id] = false;
    macros::BURN_COLOR_NORMAL(fighter);
    macros::COL_NORMAL(fighter);
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), true, true);   
    ModelModule::disable_gold_eye(fighter.module_accessor);	       
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        lightning_button, 
        //lightning_status
    );
    install_status_scripts!(
        //status_lightning_pre
    );
}  