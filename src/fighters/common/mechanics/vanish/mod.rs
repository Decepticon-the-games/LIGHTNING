use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::Vector2f;
use smashline::*;
use smash_script::*;
//use smash::phx::Hash40;
use crate::fighters::common::function_hooks::cross_cancel_vanish::PROJECTILE_HIT;
use crate::fighters::common::function_hooks::cross_cancel_vanish::DIRECT_HIT;
use crate::fighters::common::mechanics::attack_cancels::ATTACK_CANCEL;


pub static mut VANISH : [bool; 8] = [false; 8];
pub static mut VANISH_READY : [bool; 8] = [false; 8];
pub static mut VANISH_BUTTON : [bool; 8] = [false; 8]; // Only used to replicate a button_pad_trigger, runs only 1 frame
pub static mut CAN_VANISH : [bool; 8] = [false; 8];//Incorporating the ability to use vanish under condition. See lightning_01_motioncancels/mod.rs
pub static mut VANISH_COUNTER : [bool; 8] = [false; 8];
pub static mut VANISH_COUNT : [i32; 8] = [0; 8];//See motioncancels/mod.rs
pub static mut VANISH_RESET : [bool; 8] = [false; 8];//See motioncancels/mod.rs
pub static mut CANCEL_INTO_VANISH : [bool; 8] = [false; 8];
static mut CAMERA : [bool; 8] = [false; 8];
pub static mut VA_OPPONENT_X : [f32; 8] = [0.0; 8];
pub static mut VA_OPPONENT_Y : [f32; 8] = [0.0; 8];
pub static mut VA_OPPONENT_BOMA  : [u64; 8] = [0; 8];
static mut YOU_X : [f32; 8] = [0.0; 8];
static mut YOU_Y : [f32; 8] = [0.0; 8];
static mut VANISH_TIMER : [f32; 8] = [0.0; 8];
pub static mut ACTIVATE_VANISH : [bool; 8] = [false; 8];
static mut VA_OPPONENT_DIRECTION_Y : [f32; 8] = [12.0; 8];
static mut VA_OPPONENT_DIRECTION_X : [f32; 8] = [12.0; 8];
//pub static mut WHO_GOT_HIT : [i32; 8] = [0; 8];
pub static mut WHO_GOT_HIT_BOMA : [u32; 8] = [0; 8];
pub static mut GET_CURRENT_POSITION : [bool; 8] = [false; 8];
pub static mut LEFT : [bool; 8] = [false; 8];
pub static mut RIGHT : [bool; 8] = [false; 8];
pub static mut UP : [bool; 8] = [false; 8];
pub static mut DOWN : [bool; 8] = [false; 8];
static mut EFFECTS_ON : [bool; 8] = [false; 8];
static mut EFFECTS_OFF : [bool; 8] = [false; 8];

// VANISH

    #[fighter_frame_callback]
    pub fn vanish(fighter : &mut L2CFighterCommon) {
        unsafe {
            //module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
            let status_kind = StatusModule::status_kind(fighter.module_accessor);
            let frame = MotionModule::frame(fighter.module_accessor);
            //let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
            let opponent_boma = sv_battle_object::module_accessor(WHO_GOT_HIT_BOMA[entry_id]);
            let l_stick_out = ControlModule::get_stick_x(fighter.module_accessor) > 0.8|| ControlModule::get_stick_x(fighter.module_accessor) < -0.8 || ControlModule::get_stick_y(fighter.module_accessor) > 0.8 || ControlModule::get_stick_y(fighter.module_accessor) < -0.8;
            let popo_nana = (fighter_kind == *FIGHTER_KIND_POPO || fighter_kind == *FIGHTER_KIND_NANA);
            let vanish_conditions = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STATUS) || StopModule::is_hit(fighter.module_accessor);

            if entry_id < 8 { 

                
                
                   
                // Reset Vars

                if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false { //Also include a false for if the opponent is gonna be ko'ed
                    
                    VANISH_READY[entry_id] = false;
                    VANISH[entry_id] = false;
                    VANISH_BUTTON[entry_id] = false;
                    //ACTIVATE_VANISH[entry_id] = true;
                    
                }
                if status_kind == *FIGHTER_STATUS_KIND_DEAD {
                    //ACTIVATE_VANISH[entry_id] = false;
                    VANISH_READY[entry_id] = false;
                    VANISH[entry_id] = false;
                    VANISH_BUTTON[entry_id] = false;
                }

                //println!("cancelv: {}",CANCEL_INTO_VANISH[entry_id]);
                //println!("vready: {}", VANISH_READY[entry_id]);    
                //println!("phit: {}", PROJECTILE_HIT[entry_id]);
                
                    if vanish_conditions  {
                       VANISH_READY[entry_id] = false;
                    }

                    if PROJECTILE_HIT[entry_id] && frame == 10.0 {
                        PROJECTILE_HIT[entry_id] = false;
                    }  

                    if ! WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STATUS) {
                        if CANCEL_INTO_VANISH[entry_id] {                                        
                            if popo_nana {
                                VANISH_READY[entry_id] = false;   
                            }
                            else {
                                VANISH_READY[entry_id] = true; 
                            }
                        }
                        else {
                            VANISH_READY[entry_id] = false;  
                        }    
                        
                    }
                    
                


                    if VANISH_READY[entry_id] 
                    && CAN_VANISH[entry_id] 
                    {

                        //Set the directional input before pressing the vanish button
                        
                        
                        


                            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
                            { 
                                VANISH_BUTTON[entry_id] = true;// Only used to replicate a button_pad_trigger, runs only 1 frame pt 1
                            }
                            else {
                                //PROJECTILE_HIT[entry_id] = false; 
                                VANISH_BUTTON[entry_id] = false;
                            }   

                            if l_stick_out {

                                if ControlModule::get_stick_x(fighter.module_accessor) > 0.8 { //RIGHT 
                                    RIGHT[entry_id] = true;
                                }
                                else if ControlModule::get_stick_x(fighter.module_accessor) < -0.8 {//LEFT
                                    LEFT[entry_id] = true;
                                } 
                                else if ControlModule::get_stick_y(fighter.module_accessor) > 0.8 { //UP 
                                    UP[entry_id] = true;
                                }
                                else if ControlModule::get_stick_y(fighter.module_accessor) < -0.8 {//DOWN
                                    DOWN[entry_id] = true;
                                } 
                            }
                            
                            
                            

                        

                        if VANISH_BUTTON[entry_id] {// Only used to replicate a button_pad_trigger, runs only 1 frame pt 2
                            
                            if ! SlowModule::is_slow(fighter.module_accessor)
                            {
                                GET_CURRENT_POSITION[entry_id] = true;
                                
                            }
                            
                        }
                    }

                

                if GET_CURRENT_POSITION[entry_id] { //RECALL PLAYER ID VARIABLE, and store their position

                    VA_OPPONENT_X[entry_id] = PostureModule::pos_x(opponent_boma);
                    VA_OPPONENT_Y[entry_id] = PostureModule::pos_y(opponent_boma);
                    VA_OPPONENT_BOMA[entry_id] = (&mut *opponent_boma as *mut BattleObjectModuleAccessor) as u64;
                    if utility::get_category(&mut *opponent_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                        JostleModule::set_status(&mut *opponent_boma, false);
                    }
                    VANISH[entry_id] = true; 
                    VANISH_READY[entry_id] = false; 
                    GET_CURRENT_POSITION[entry_id] = false;
                }
                // VANISH

                
                if VANISH[entry_id] {

                    VANISH_BUTTON[entry_id] = false;
                    EFFECTS_ON[entry_id] = true;

                    if CAMERA[entry_id] == false { // Exists so all of this code will only happen once.


                        YOU_X[entry_id] = PostureModule::pos_x(fighter.module_accessor); // Gets Ryu's position
                        YOU_Y[entry_id] = PostureModule::pos_y(fighter.module_accessor);

                        //if l_stick_out {// if directional input, affect the destination of the vanish
                            
                            if RIGHT[entry_id] { //RIGHT 
                                if YOU_X[entry_id] < VA_OPPONENT_X[entry_id] { // If you're left of the opponent
                                    VA_OPPONENT_DIRECTION_X[entry_id] = -12.0; //appear to the left of the opponent
                                    VA_OPPONENT_DIRECTION_Y[entry_id] = 0.0;
                                }
                                if YOU_X[entry_id] > VA_OPPONENT_X[entry_id]{// If you're right of the opponent
                                    VA_OPPONENT_DIRECTION_X[entry_id] = -12.0; //appear to the left of the opponent
                                    if PostureModule::lr(fighter.module_accessor) == -1.0 {
                                        PostureModule::reverse_lr(fighter.module_accessor);                                    
                                    }
                                    VA_OPPONENT_DIRECTION_Y[entry_id] = 0.0;
                                }
                                RIGHT[entry_id] = false;

                            }
                            else if LEFT[entry_id] { //LEFT
                                if YOU_X[entry_id] > VA_OPPONENT_X[entry_id] { // If you're right of the opponent
                                    VA_OPPONENT_DIRECTION_X[entry_id] = 12.0; //appear to the right of the opponent
                                    VA_OPPONENT_DIRECTION_Y[entry_id] = 0.0;
                                }
                                if YOU_X[entry_id] < VA_OPPONENT_X[entry_id] {// If you're left of the opponent
                                    VA_OPPONENT_DIRECTION_X[entry_id] = 12.0; //appear to the right of the opponent                                        
                                    if PostureModule::lr(fighter.module_accessor) == 1.0 {
                                        PostureModule::reverse_lr(fighter.module_accessor);
                                    }
                                    VA_OPPONENT_DIRECTION_Y[entry_id] = 0.0;
                                }
                                LEFT[entry_id] =  false;

                            }
                        
                        
                            else if UP[entry_id] { //UP
                                
                                if YOU_Y[entry_id] < VA_OPPONENT_Y[entry_id] { // If you're below the opponent
                                    VA_OPPONENT_DIRECTION_Y[entry_id] = -20.0; //appear below the opponent
                                    VA_OPPONENT_DIRECTION_X[entry_id] = 0.0;
                                }
                                else if YOU_Y[entry_id] > VA_OPPONENT_Y[entry_id]{ // If you're above the opponent
                                    VA_OPPONENT_DIRECTION_Y[entry_id] = - 20.0; //appear below the opponent
                                    VA_OPPONENT_DIRECTION_X[entry_id] = 0.0;
                                }  
                                UP[entry_id] = false;
                            }
                            else if DOWN[entry_id] { //DOWN


                                if YOU_Y[entry_id] > VA_OPPONENT_Y[entry_id] { // If you're above the opponent
                                    VA_OPPONENT_DIRECTION_Y[entry_id] = 20.0; //appear above the opponent
                                    VA_OPPONENT_DIRECTION_X[entry_id] = 0.0;
                                }
                                else if YOU_Y[entry_id] < VA_OPPONENT_Y[entry_id]{ // If you're below the opponent
                                    VA_OPPONENT_DIRECTION_Y[entry_id] = 28.0; //appear above the opponent
                                    VA_OPPONENT_DIRECTION_X[entry_id] = 0.0;
                                }
                                DOWN[entry_id] = false;

                            }                            
                            else { //If no direction, default to behind the opponent
                                if YOU_X[entry_id] < VA_OPPONENT_X[entry_id] {// If you're left of the opponent
                                    VA_OPPONENT_DIRECTION_X[entry_id] = 12.0; //appear to the right of the opponent                                        
                                    if PostureModule::lr(fighter.module_accessor) == 1.0 {
                                        PostureModule::reverse_lr(fighter.module_accessor);
                                    }
                                    VA_OPPONENT_DIRECTION_Y[entry_id] = 0.0;
                                }
                                else if YOU_X[entry_id] > VA_OPPONENT_X[entry_id]{// If you're right of the opponent
                                    VA_OPPONENT_DIRECTION_X[entry_id] = -12.0; //appear to the left of the opponent
                                    if PostureModule::lr(fighter.module_accessor) == -1.0 {
                                        PostureModule::reverse_lr(fighter.module_accessor);                                    
                                    }
                                    VA_OPPONENT_DIRECTION_Y[entry_id] = 0.0;
                                }
                            }                                                         
                        //}



                        CAMERA[entry_id] = true; // Again, ensures that the above code only runs once.
                    }

                    
                    if VANISH_TIMER[entry_id] >= 0.0 { // This whole if statement is for linearly interpolating your position, instead of just teleporting him behind the opponent.
                        if (YOU_Y[entry_id] - VA_OPPONENT_Y [entry_id]).abs() > 12.0 { // If yuor Y and the Opponent's Y are far enough apart, do the following:
                            StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), false); // Set yourself to be airborne
                        }
                        PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f{ // Linear Interpolation formula: Destination * t + Starting * (1.0 - t), where 0 <= t <= 1. You can't add vectors apparently, so I did this for both X and Y.
                            x: (((VA_OPPONENT_X[entry_id] + VA_OPPONENT_DIRECTION_X[entry_id]) * VANISH_TIMER[entry_id]) + YOU_X[entry_id] * (1.0 - VANISH_TIMER[entry_id])),
                            y: (((VA_OPPONENT_Y [entry_id] + VA_OPPONENT_DIRECTION_Y[entry_id]) * VANISH_TIMER[entry_id]) + YOU_Y[entry_id] * (1.0 - VANISH_TIMER[entry_id]))
                        });
                    }
                    VANISH_TIMER[entry_id] += 0.1; // Increases the "t" in the interpolation formula by 0.1 every frame.
                    if VANISH_TIMER[entry_id] > 1.0 {
                        VANISH[entry_id] = false;
                        CAMERA[entry_id] = false;
                        EFFECTS_OFF[entry_id] = true;


                        VANISH_TIMER[entry_id] = 0.0; // Resets the interpolation timer.
                        VANISH_COUNTER[entry_id] = true;
                        
                    }
                    
                }

                else {
                    VANISH[entry_id] = false;
                    //ACTIVATE_VANISH[entry_id] = true;
                    //macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
                }

                //EFFECTS ON/OFF
                if EFFECTS_ON[entry_id] {


                    if ! status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK { //Don't turn around on grab attack
                        PostureModule::reverse_lr(fighter.module_accessor); 
                    }
                    if status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK { //Make the opponent invisible too, for the illusion of vanish
                        VisibilityModule::set_whole(opponent_boma, false);
                    }

   
                    EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_attack_speedline"), smash::phx::Hash40::new("body"), &smash::phx::Vector3f {x:0.0,y:0.0,z:0.0}, &smash::phx::Vector3f {x:0.0,y:0.0,z:90.0}, 1.0, &smash::phx::Vector3f {x:0.0,y:0.0,z:0.0}, &smash::phx::Vector3f {x:0.0,y:0.0,z:0.0}, false, 0, 0, 0);
                    //macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);             
                    //EffectModule::req_emit(fighter.module_accessor, Hash40::new("sys_aura_light"), 0);
                    //macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
                    VisibilityModule::set_whole(fighter.module_accessor, false);
                    JostleModule::set_status(fighter.module_accessor, false); // Turns off body blocking for Ryu every frame Secret Sensation is true
                    //macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU); // Makes Ryu invincible.
                    MotionModule::set_rate(fighter.module_accessor, 0.001);
                    macros::SLOW_OPPONENT(fighter, 100.0, 10.0);
                    EFFECTS_ON[entry_id] = false;
                }
                if EFFECTS_OFF[entry_id] {

                    if (fighter_kind == *FIGHTER_KIND_POPO || fighter_kind == *FIGHTER_KIND_NANA || fighter_kind == *FIGHTER_KIND_ICE_CLIMBER) {
                        VisibilityModule::set_whole(fighter.module_accessor, true);
                    }
                    else {
                        VisibilityModule::set_whole(opponent_boma, true);                       
                    }
                    EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_attack_speedline"), smash::phx::Hash40::new("body"), &smash::phx::Vector3f {x:0.0,y:0.0,z:0.0}, &smash::phx::Vector3f {x:0.0,y:0.0,z:90.0}, 1.0, &smash::phx::Vector3f {x:0.0,y:0.0,z:0.0}, &smash::phx::Vector3f {x:0.0,y:0.0,z:0.0}, false, 0, 0, 0);
                    SlowModule::clear_whole(fighter.module_accessor);
                    MotionModule::set_rate(fighter.module_accessor, 1.0); 
                    JostleModule::set_status(fighter.module_accessor, true);                        
                    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE); 
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK); 
                    macros::SET_SPEED_EX(fighter, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                    
                    if status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK { //Make the opponent invisible too, for the illusion of vanish
                        VisibilityModule::set_whole(opponent_boma, true);
                    }
                    //VisibilityModule::set_whole(opponent_boma, true);    
                    EFFECTS_OFF[entry_id] = false;                
                }

            }
        }
    }

pub fn install() {

   
    smashline::install_agent_frame_callbacks!(vanish);
}