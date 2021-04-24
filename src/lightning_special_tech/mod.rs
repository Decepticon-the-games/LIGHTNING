use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use acmd::*;


// CREATED BY PHAZOGANON, THANK YOU :)

// LIGHTNING_CANCEL_TIMER triggers faster motion rate for attacks and specials//

pub static mut TIME_SLOW_EFFECT_VECTOR: smash::phx::Vector3f = smash::phx::Vector3f {x:-3.0,y:3.0,z:0.0};
pub const TIME_SLOW_EFFECT_HASH: u64 = smash::hash40("sys_sp_flash");
static mut LIGHTNING_CANCEL : [bool; 8] = [false;  8];
static mut LIGHTNING_CANCEL_TIMER : [i32; 8] = [-1;  8];
static mut CRIMSON_CANCELLING : [i32; 8] = [-1;  8];
static mut CAN_CRIMSON_CANCEL : [bool; 8] = [true; 8];
static mut CAN_CRIMSON_CANCEL_TEMP : [bool; 8] = [true; 8];
    
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let lua_state = fighter.lua_state_agent; 
        //let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        //let cat3 = ControlModule::get_command_flag_cat(module_accessor, 2);
        
        //LIGHTNING_CANCEL (LIGHTNING CANCELLING)
        
        if LIGHTNING_CANCEL_TIMER[entry_id] == -1
        && LIGHTNING_CANCEL[entry_id] == false {

            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)  {
               
                StatusModule::delete_status_request_from_script(module_accessor);

                LIGHTNING_CANCEL_TIMER[entry_id] = 1800; 
                LIGHTNING_CANCEL[entry_id] = true;
                
                    
                    
                   
                EffectModule::req_on_joint(module_accessor, smash::phx::Hash40::new_raw(TIME_SLOW_EFFECT_HASH), smash::phx::Hash40::new("head"), &TIME_SLOW_EFFECT_VECTOR, &TIME_SLOW_EFFECT_VECTOR, 1.0, &TIME_SLOW_EFFECT_VECTOR, &TIME_SLOW_EFFECT_VECTOR, false, 0, 0, 0); 
            }
            
        }
        

        if LIGHTNING_CANCEL_TIMER[entry_id] >= 0 {     
            
            if LIGHTNING_CANCEL_TIMER[entry_id] >= 1740 {
                if  MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_lw_l") || MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_lw_r") {
                    CancelModule::enable_cancel(module_accessor);
                }
                //DamageModule::add_damage(module_accessor, -5.0, 0); 
            }
            
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S3
            || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_HI3
            || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_LW3
            || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S4
            || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_HI4
            || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_LW4
            || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_AIR
            || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_N
            || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_S
            || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                
                if ! AttackModule::is_attack(module_accessor, 0, false) {
                  MotionModule::set_rate(module_accessor, 1.1);
                }

            }
            AttackModule::set_reaction_mul(module_accessor, 0.9); 
            LIGHTNING_CANCEL_TIMER[entry_id] -= 1;   
                
        }
        
        //_________________________________________________________________________________________________________________________________________________________________________________     
        
        // CRIMSON CANCELLING (DPAD UP) For 2 seconds the opponent will slow down by 5 times, two players can't use them at the same time

            if CRIMSON_CANCELLING[entry_id] == -1 
            && CAN_CRIMSON_CANCEL[entry_id] {
                
                if DamageModule::damage(module_accessor, 0) >= 50.0 {
                    
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                        CRIMSON_CANCELLING[entry_id] = 120;
                        EffectModule::req_on_joint(module_accessor, smash::phx::Hash40::new_raw(TIME_SLOW_EFFECT_HASH), smash::phx::Hash40::new("head"), &TIME_SLOW_EFFECT_VECTOR, &TIME_SLOW_EFFECT_VECTOR, 1.0, &TIME_SLOW_EFFECT_VECTOR, &TIME_SLOW_EFFECT_VECTOR, false, 0, 0, 0); 
                        acmd!(lua_state,{
                                
                            SLOW_OPPONENT(5, 120)
                            FILL_SCREEN_MODEL_COLOR( 0, 12, 0.1, 0.1, 0.1, 0.01, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);

                        });
                        for mut _x in CAN_CRIMSON_CANCEL.iter() {
                            _x = &false;
                        }
                        CAN_CRIMSON_CANCEL_TEMP = CAN_CRIMSON_CANCEL;
                    
                        
                    }
                    
                }
            }

            if CRIMSON_CANCELLING[entry_id] >= 60 {
                if  MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_hi_l") || MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_hi_r") {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
            
            if CRIMSON_CANCELLING[entry_id] >= 1 {
                
                CRIMSON_CANCELLING[entry_id] -=1;
            }
            if CRIMSON_CANCELLING[entry_id] == 0 {
                acmd!(lua_state,{
                    CANCEL_FILL_SCREEN(0, 5) 
                });
                CAN_CRIMSON_CANCEL = CAN_CRIMSON_CANCEL_TEMP;
            }
        //_________________________________________________________________________________________________________________________________________________________________________________    
 

        //RESET EACH STOCK
        
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
            LIGHTNING_CANCEL[entry_id] = false;
            LIGHTNING_CANCEL_TIMER[entry_id] = -1;
            CRIMSON_CANCELLING[entry_id] = -1;
            CAN_CRIMSON_CANCEL[entry_id] = true;
        } 
    }
}

         

                                         


pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
}