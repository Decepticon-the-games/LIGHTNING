// use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;

// CREATED BY PHAZOGANON, THANK YOU :)

// LIGHTNING_CANCEL_TIMER triggers faster motion rate for attacks and specials//

pub static mut TIME_SLOW_EFFECT_VECTOR: smash::phx::Vector3f = smash::phx::Vector3f {x:-3.0,y:3.0,z:0.0};
pub const TIME_SLOW_EFFECT_HASH: u64 = smash::hash40("sys_sp_flash");
//static mut LIGHTNING_CANCEL : [bool; 8] = [false;  8];
//static mut LIGHTNING_CANCEL_TIMER : [i32; 8] = [-1;  8];
//pub const FINAL_AURA_HASH: u64 = smash::hash40("sys_final_aura");
static mut CRIMSON_CANCELLING : [i32; 8] = [-1;  8];
static mut CAN_CRIMSON_CANCEL : [bool; 8] = [true; 8];
static mut CAN_CRIMSON_CANCEL_TEMP : [bool; 8] = [true; 8];
//pub static mut SPECIAL_N_FINAL_SMASH_METER : [bool; 8] = [false; 8];

#[fighter_frame_callback]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let lua_state = fighter.lua_state_agent; 
        let status_kind = StatusModule::status_kind(module_accessor);
        let situation_kind = StatusModule::situation_kind(module_accessor);
        //let mut gfxname: [&str; 8] = ["sys_final_aura"; 8];
        //let gfxcoords  = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
        //let mut gfxsize: [f32; 8] = [0.15; 8];
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        //let cat3 = ControlModule::get_command_flag_cat(module_accessor, 2);
        
        //LIGHTNING_CANCEL (LIGHTNING CANCELLING)

        //if LIGHTNING_CANCEL_TIMER[entry_id] == -1
        //&& LIGHTNING_CANCEL[entry_id] == false {

        
            if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL) {

                //LIGHTNING_CANCEL_TIMER[entry_id] = 20; 
                //LIGHTNING_CANCEL[entry_id] = true;

                //Take Less Damage Knockback
                if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N != 0 {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
                }
                //DamageModule::heal(module_accessor, 0.01, 0);
                //acmd!(lua_state,{
                //    CANCEL_FILL_SCREEN(0, 5)
                //});

                    
                //RESET JUMP ON HIT EXCEPT UP SPECIAL OF ALL KINDS
        
                if situation_kind == *SITUATION_KIND_AIR {
                    if ! (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
                    || status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A
                    || status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G
                    || status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP ) {
                        if AttackModule::is_attack_occur(module_accessor) {
                            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP){
                                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                            }
                        }   
                    }
                }

                
                //ALL Attack move move a bit fasteer
                
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3
                || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3
                || status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3
                || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4
                || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4
                || status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4
                || status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR
                || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N
                || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
                || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                    
                    if ! AttackModule::is_attack(module_accessor, 0, false) {
                    MotionModule::set_rate(module_accessor, 1.5);
                    }

                } 
                
                //if LIGHTNING_CANCEL_TIMER[entry_id] >= 0 {
                    
                //    LIGHTNING_CANCEL_TIMER[entry_id] -= 1;
                //}
            }
        //}
        
        //_________________________________________________________________________________________________________________________________________________________________________________     
        
        // CRIMSON CANCELLING (DPAD UP) For 2 seconds the opponent will slow down by 5 times, two players can't use them at the same time

            if CRIMSON_CANCELLING[entry_id] == -1 
            && CAN_CRIMSON_CANCEL[entry_id] {
                
                if DamageModule::damage(module_accessor, 0) >= 50.0 {
                    
                    if ControlModule::get_command_flag_cat(module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0 {
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
            if CRIMSON_CANCELLING[entry_id] == 0 || status_kind == *FIGHTER_STATUS_KIND_DEAD {
                acmd!(lua_state,{
                    CANCEL_FILL_SCREEN(0, 5) 
                });
                CAN_CRIMSON_CANCEL = CAN_CRIMSON_CANCEL_TEMP;
            }
        //_________________________________________________________________________________________________________________________________________________________________________________    
 

        //RESET EACH STOCK
        
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false  {
            CRIMSON_CANCELLING[entry_id] = -1;
            CAN_CRIMSON_CANCEL[entry_id] = true;
            //LIGHTNING_CANCEL_TIMER[entry_id] = -1; 
            //LIGHTNING_CANCEL[entry_id] = false;
        } 
    }
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(once_per_fighter_frame);
}