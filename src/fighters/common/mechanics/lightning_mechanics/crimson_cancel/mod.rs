use super::*;


// CREATED BY PHAZOGANON, THANK YOU :)

// CRIMSON CANCELLING (DPAD UP) For 2 seconds the opponent will slow down by 5 times, two players can't use them at the same time

pub static mut TIME_SLOW_EFFECT_VECTOR: smash::phx::Vector3f = smash::phx::Vector3f {x:-3.0,y:3.0,z:0.0};
pub const TIME_SLOW_EFFECT_HASH: u64 = smash::hash40("sys_sp_flash");
pub static mut FIGHTER_INSTANCE_WORK_ID_FLAG_CRIMSON_CANCEL : [bool; 8] = [false; 8];
pub static mut CRIMSON_CANCEL_TIMER : [i32; 8] = [-1; 8];
pub static mut CAN_CRIMSON_CANCEL : [bool; 8] = [true; 8];//the ability to crimson cancel
static mut CAN_CRIMSON_CANCEL_TEMP : [bool; 8] = [true; 8];
pub static mut CRIMSON_CANCEL_BUTTON : [bool; 8] = [false; 8];//run only 1 frame
pub static mut CRIMSON_CANCEL_EFFECTS : [bool; 8] = [false; 8];


#[fighter_frame_callback]
pub fn crimson_cancel_button(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        let hitlag = (SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) > 0 || StopModule::is_stop(fighter.module_accessor));
    
        let idles =  (status_kind == *FIGHTER_STATUS_KIND_WAIT || status_kind == *FIGHTER_STATUS_KIND_JUMP || status_kind == *FIGHTER_STATUS_KIND_FALL);
            
        if crimson_cancel_conditions(fighter) {

            if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW) != 0 
            // && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_APPEAL_L) != 0
            && (idles || is_after_hitlag(fighter))
            {
                CRIMSON_CANCEL_BUTTON[entry_id] = true;
            }
            else {
                CRIMSON_CANCEL_BUTTON[entry_id] = false; 
            }
        }
        if CRIMSON_CANCEL_BUTTON[entry_id] {

            crimson_cancel(fighter);
            CAN_CRIMSON_CANCEL= CAN_CRIMSON_CANCEL_TEMP;
            CAN_CRIMSON_CANCEL[entry_id] = false;
            CRIMSON_CANCEL_BUTTON[entry_id] = false;                    

        }
            
        //RESET EACH STOCK
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
            CRIMSON_CANCEL_TIMER[entry_id] = -1;
            CAN_CRIMSON_CANCEL[entry_id] = true;
        } 
    }
}
pub unsafe fn crimson_cancel_conditions(fighter : &mut L2CFighterCommon) -> bool {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let hitlag = (SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) > 0 || StopModule::is_stop(fighter.module_accessor));

    CRIMSON_CANCEL_TIMER[entry_id] == -1 
    && CAN_CRIMSON_CANCEL[entry_id]
    && ! FIGHTER_STATUS_KIND_VANISH[entry_id]    
    && DamageModule::damage(fighter.module_accessor, 0) >= 50.0 
    // WHEN THE METER COMES, CHANGE IT TO METER IS +50??   
    && ! (CaptureModule::is_capture(fighter.module_accessor))
    && ! (StopModule::is_hit(fighter.module_accessor))

    //Can't spark while being hit in hitlag or being held in a grab/throw, wastes it 
}
#[fighter_frame_callback]
pub fn crimson_cancel_timer(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let hitlag = (AttackModule::is_attack_occur(fighter.module_accessor) && SlowModule::is_slow(fighter.module_accessor));
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        //Countdown sequence
        if CRIMSON_CANCEL_TIMER[entry_id] >= 1 {
            CRIMSON_CANCEL_TIMER[entry_id] -=1;
        }  
        //When you attack next, the timer runs out or you get KO'd, the effects wear off
        if CRIMSON_CANCEL_TIMER[entry_id] == 0 
        || status_kind == *FIGHTER_STATUS_KIND_DEAD 
        || (CRIMSON_CANCEL_TIMER[entry_id]  >=1 && (hitlag)) 
        {
            CAN_CRIMSON_CANCEL_TEMP = CAN_CRIMSON_CANCEL;
            CRIMSON_CANCEL_TIMER[entry_id] = -1;
            crimson_cancel_disable(fighter);
        }
    }
}
pub unsafe fn crimson_cancel(fighter : &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    
    FIGHTER_INSTANCE_WORK_ID_FLAG_CRIMSON_CANCEL[entry_id] = true;
    CRIMSON_CANCEL_TIMER[entry_id] = 120;
    
    crimson_cancel_effects(fighter);

    if CRIMSON_CANCEL_BUTTON[entry_id] {
        CRIMSON_CANCEL_BUTTON[entry_id] = false; 
    }

}
pub fn crimson_cancel_disable(fighter : &mut L2CFighterCommon) {
    unsafe {  
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        FIGHTER_INSTANCE_WORK_ID_FLAG_CRIMSON_CANCEL[entry_id] = false;
        macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
        macros::CANCEL_FILL_SCREEN(fighter, 0, 5.0);
        macros::SLOW_OPPONENT(fighter, 0.0, 0.0);
        macros::EFFECT_OFF_KIND(fighter, smash::phx::Hash40::new("sys_aura_dark"), true, true);
    }
}
pub fn crimson_cancel_effects(fighter : &mut L2CFighterCommon) {
    unsafe {       
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        
        //Visual Effects    
        EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new_raw(TIME_SLOW_EFFECT_HASH), smash::phx::Hash40::new("head"), &TIME_SLOW_EFFECT_VECTOR, &TIME_SLOW_EFFECT_VECTOR, 1.0, &TIME_SLOW_EFFECT_VECTOR, &TIME_SLOW_EFFECT_VECTOR, false, 0, 0, 0);
        EffectModule::req_emit(fighter.module_accessor, smash::phx::Hash40::new("sys_aura_dark"), 0);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.773, 0.031, 0.304);
        if FIGHTER_STATUS_KIND_VANISH[entry_id] == false {
           macros::SLOW_OPPONENT(fighter, 5.0, 120.0); 
        }
        macros::FILL_SCREEN_MODEL_COLOR(fighter, 0, 12, 0.1, 0.1, 0.1, 0.01, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);
        macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);

        //Dimensional Effects
        //JostleModule::set_status(fighter.fighter.module_accessor, false);// Go through an opponent
        //WorkModule::on_flag(fighter.fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);// gravity
        // CHANGE THE STAGE BACKGROUND!!!!!!!!! 
        // MAKE THE STAGE ITSELF TRANSPARENT!!!!!!!!!!
        //_________________________________________________________________________________________________________________________________________________________________________________    
    }
}



    
    
pub fn install() {
    smashline::install_agent_frame_callbacks!(crimson_cancel_button, crimson_cancel_timer);
}