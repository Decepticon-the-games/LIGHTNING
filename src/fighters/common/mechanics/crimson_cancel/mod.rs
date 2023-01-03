//use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;

use smash_script::*;

// CREATED BY PHAZOGANON, THANK YOU :)

// CRIMSON CANCELLING (DPAD UP) For 2 seconds the opponent will slow down by 5 times, two players can't use them at the same time

pub static mut TIME_SLOW_EFFECT_VECTOR: smash::phx::Vector3f = smash::phx::Vector3f {x:-3.0,y:3.0,z:0.0};
pub const TIME_SLOW_EFFECT_HASH: u64 = smash::hash40("sys_sp_flash");
pub static mut CRIMSON_CANCEL_TIMER : [i32; 8] = [-1;  8];
pub static mut CAN_CRIMSON_CANCEL : [bool; 8] = [true; 8];//the ability to crimson cancel
static mut CAN_CRIMSON_CANCEL_TEMP : [bool; 8] = [true; 8];
pub static mut CRIMSON_CANCEL_BUTTON : [bool; 8] = [false; 8];//run only 1 frame
pub static mut CRIMSON_CANCEL_EFFECTS : [bool; 8] = [false; 8];
pub static mut DISABLE_ATTACKING : [bool; 8] = [false; 8];

#[fighter_frame_callback]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(module_accessor);
        let hitlag = (AttackModule::is_attack_occur(module_accessor) && SlowModule::is_slow(module_accessor));

        if CRIMSON_CANCEL_TIMER[entry_id] == -1 
        && CAN_CRIMSON_CANCEL[entry_id] {
            
            if DamageModule::damage(module_accessor, 0) >= 50.0 {
                if ! (CaptureModule::is_capture(module_accessor))
                && ! (StopModule::is_hit(module_accessor))
                && ! (hitlag)
                { //Can't spark while atacking in hitlag, being hit in hitlag or being held in a grab/throw, wastes it 
                    
                    
                    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) 
                    //&& ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) 
                    {
                        
                        CRIMSON_CANCEL_BUTTON[entry_id] = true;
                        CAN_CRIMSON_CANCEL_TEMP = CAN_CRIMSON_CANCEL;
                        CAN_CRIMSON_CANCEL = [false; 8];
                    }
                }                
            }

            
        }

        if CRIMSON_CANCEL_BUTTON[entry_id] 
        //&& CancelModule::is_enable_cancel(fighter.module_accessor)
        {
            CRIMSON_CANCEL_EFFECTS[entry_id] = true;
            CRIMSON_CANCEL_BUTTON[entry_id] = false;

        }
        if CRIMSON_CANCEL_EFFECTS[entry_id] {

            //Visual Effects
            CRIMSON_CANCEL_TIMER[entry_id] = 120;
            EffectModule::req_on_joint(module_accessor, smash::phx::Hash40::new_raw(TIME_SLOW_EFFECT_HASH), smash::phx::Hash40::new("head"), &TIME_SLOW_EFFECT_VECTOR, &TIME_SLOW_EFFECT_VECTOR, 1.0, &TIME_SLOW_EFFECT_VECTOR, &TIME_SLOW_EFFECT_VECTOR, false, 0, 0, 0);
            EffectModule::req_emit(module_accessor, smash::phx::Hash40::new("sys_aura_dark"), 0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.773, 0.031, 0.304);
            macros::SLOW_OPPONENT(fighter, 5.0, 120.0);
            macros::FILL_SCREEN_MODEL_COLOR(fighter, 0, 12, 0.1, 0.1, 0.1, 0.01, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);

            //Dimensional Effects
            //JostleModule::set_status(fighter.module_accessor, false);// Go through an opponent
            //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);// gravity


            CRIMSON_CANCEL_EFFECTS[entry_id] = false;

        }
        if CRIMSON_CANCEL_TIMER[entry_id] >= 60 
        && (MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_lw_l") || MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_lw_r")) 
        && ! ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {    
            crate::fighters::common::utility::enable_cancel_real(fighter); //Cancels the taunt to gain maximum spark time   
        }
        //Countdown sequence
        if CRIMSON_CANCEL_TIMER[entry_id] >= 1 {
            CRIMSON_CANCEL_TIMER[entry_id] -=1;
            //DISABLE_ATTACKING[entry_id] = true;
            macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU); // Makes Ryu invincible./invincible to any attacks
            //AttackModule::clear_all(module_accessor);
        }            

        
        //When you attack next, the timer runs out or you get KO'd, the effects wear off
        if CRIMSON_CANCEL_TIMER[entry_id] == 0 || status_kind == *FIGHTER_STATUS_KIND_DEAD || (hitlag) {
            CAN_CRIMSON_CANCEL = CAN_CRIMSON_CANCEL_TEMP;
            macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
            macros::CANCEL_FILL_SCREEN(fighter, 0, 5.0);
            macros::SLOW_OPPONENT(fighter, 0.0, 0.0);
            macros::EFFECT_OFF_KIND(fighter, smash::phx::Hash40::new("sys_aura_dark"), true, true);
            
        }

        // CHANGE THE STAGE BACKGROUND!!!!!!!!! 
        // MAKE THE STAGE ITSELF TRANSPARENT!!!!!!!!!!
        //_________________________________________________________________________________________________________________________________________________________________________________    
 

        //RESET EACH STOCK
        
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false  {
            CRIMSON_CANCEL_TIMER[entry_id] = -1;
            CAN_CRIMSON_CANCEL[entry_id] = true;
        } 
    }
}
    
    
pub fn install() {
    smashline::install_agent_frame_callbacks!(once_per_fighter_frame);
}