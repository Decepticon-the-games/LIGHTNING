use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::Vector2f;
use smashline::*;
use smash_script::*;
//use smash::phx::Hash40;
//use fighters::common::mechanics::lightning_mechanics::vanish::damage_statuses::is_damage_status;
use crate::fighters::common::function_hooks::cross_cancel_vanish::PROJECTILE_HIT;
use crate::fighters::common::function_hooks::cross_cancel_vanish::DIRECT_HIT;
use crate::fighters::common::mechanics::cancels::motioncancels::CANCEL_IN_NEUTRAL;
use crate::fighters::common::mechanics::cancels::attack_cancels::{ENABLE_ATTACK_CANCEL, ATTACK_CANCEL};

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
pub static mut DISABLE_CATCH : [bool; 8] = [false; 8];
pub static mut DEFENDER_VANISH : [bool; 8] = [false; 8];
static mut VA_OPPONENT_DIRECTION_Y : [f32; 8] = [12.0; 8];
static mut VA_OPPONENT_DIRECTION_X : [f32; 8] = [12.0; 8];
static mut VANISH_DIREC : [i32; 8] = [0; 8];
pub static mut FLOAT_TIMER : [i32; 8] = [0; 8];
pub static mut WINDOW : [i32; 8] = [0; 8];
pub static mut WHO_GOT_HIT_BOMA : [u32; 8] = [0; 8];
pub static mut WHO_HIT_YOU_BOMA : [u32; 8] = [0; 8];
//pub static mut attack_vanish_get_current_position : [bool; 8] = [false; 8];
pub static mut STICK_MANIPULATION : [bool; 8] = [false; 8];
pub static mut FLOAT : [bool; 8] = [false; 8];
static mut VERT_EXTRA : [f32; 8] = [6.0; 8];
static mut FLASH_TIMER : [i16; 8] = [-1; 8]; 
//static mut EFFECTS_OFF : [bool; 8] = [false; 8];
//pub static mut VANISH_TO_ENTRY_ID : [u32; 8] = [0; 8];


//BUTTON
#[fighter_frame_callback]
pub fn vanish_button(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let dir_x = ControlModule::get_stick_x(fighter.module_accessor);
        let dir_y = ControlModule::get_stick_y(fighter.module_accessor);
        let stick_manipulation = (dir_x >= 0.8 || dir_x <= -0.8 || dir_y >= 0.8 || dir_y <= -0.8 );
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        
        if CAN_VANISH[entry_id] {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {// down taunt
                vanish_defense(fighter);   
            }
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {// grab
                vanish_attack(fighter);   
            }
        }
    }
}
#[fighter_frame_callback]
//DEFENSE VERSION
pub fn vanish_defense(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        //if CAN_VANISH[entry_id] {
            if VANISH[entry_id] == false
            {
                DEFENDER_VANISH[entry_id] = true;
                WINDOW[entry_id] += 1;
                DamageModule::set_damage_lock(fighter.module_accessor, true); // Makes sure Ryu doesn't take damage during the dodge
                DamageModule::set_no_reaction_no_effect(fighter.module_accessor, true); // Makes sure Ryu doesn't take knockback.
                HitModule::set_hit_stop_mul(fighter.module_accessor, 0.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
                if WINDOW[entry_id] < 15 {//15 frame window
                    DEFENDER_VANISH[entry_id] = false;
                    WINDOW[entry_id] -1;
                }
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
            else if DEFENDER_VANISH[entry_id] 
            && VANISH[entry_id] == false {
                DamageModule::set_damage_lock(fighter.module_accessor, false);
                DamageModule::set_no_reaction_no_effect(fighter.module_accessor, false);
                HitModule::set_hit_stop_mul(fighter.module_accessor, 1.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
                DEFENDER_VANISH[entry_id] = false;
            }
        //}
    }
}

//#[fighter_frame_callback]
//ATTACK VERSION
pub fn vanish_attack(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let opponent_boma = sv_battle_object::module_accessor(WHO_GOT_HIT_BOMA[entry_id]);

        //If an attack occurs(after hitlag), enable cancelling into vanish
        if (AttackModule::is_attack_occur(fighter.module_accessor) 
        && SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) == 0)
        && CancelModule::is_enable_cancel(fighter.module_accessor) 
        && ! StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_THROW { 

            CANCEL_INTO_VANISH[entry_id] = true;
        }  
        else {
            CANCEL_INTO_VANISH[entry_id] = false;
        }  

        if CANCEL_INTO_VANISH[entry_id] 
        && ENABLE_ATTACK_CANCEL[entry_id] == false
        && ATTACK_CANCEL[entry_id] == false {

            attack_vanish_get_current_position(fighter);
        }
    }
}
//GET OPPONENT POSITION
pub unsafe fn attack_vanish_get_current_position(fighter : &mut L2CFighterCommon) { //RECALL PLAYER ID VARIABLE, and store their position
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let opponent_boma = sv_battle_object::module_accessor(WHO_GOT_HIT_BOMA[entry_id]);

    VA_OPPONENT_X[entry_id] = PostureModule::pos_x(opponent_boma);
    VA_OPPONENT_Y[entry_id] = PostureModule::pos_y(opponent_boma);
    VA_OPPONENT_BOMA[entry_id] = (&mut *opponent_boma as *mut BattleObjectModuleAccessor) as u64;
    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
    if utility::get_category(&mut *opponent_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        JostleModule::set_status(&mut *opponent_boma, false);
    }

    VANISH[entry_id] = true;
    VANISH_READY[entry_id] = false; 
    CANCEL_INTO_VANISH[entry_id] = false;
    DISABLE_CATCH[entry_id] = false;
}
#[fighter_frame_callback]
//VANISH
pub fn vanish(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        
        let dir_x = ControlModule::get_stick_x(fighter.module_accessor);
        let dir_y = ControlModule::get_stick_y(fighter.module_accessor);
        let stick_degrees = dir_y.atan2(dir_x).to_degrees();         
        let stick_manipulation = (dir_x >= 0.8 || dir_x <= -0.8 || dir_y >= 0.8 || dir_y <= -0.8 );

        
        if entry_id < 8 {
        
            if VANISH[entry_id] {
                
                enable_vanish_effects(fighter);
                if CAMERA[entry_id] == false {
                    ControlModule::clear_command(fighter.module_accessor, true);//The buffer is cleared so you don't end up automatically airdodging out of vanish
                    YOU_X[entry_id] = PostureModule::pos_x(fighter.module_accessor); // Gets your position
                    YOU_Y[entry_id] = PostureModule::pos_y(fighter.module_accessor);

                    if stick_manipulation && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {//Manipulate position using a formula converting stickc_degrees to the point of circumference (12x/12y inversion unit radius)

                        VA_OPPONENT_DIRECTION_X[entry_id] = (dir_x * -12.0); //(stick driection * how far from the opponent's destination)
                        VA_OPPONENT_DIRECTION_Y[entry_id] = (dir_y * -12.0);                  
                        
                    }
                    else if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                        //If no stick direction and on the ground, default to behind the opponent
                        if YOU_X[entry_id] == VA_OPPONENT_X[entry_id] {
                            VA_OPPONENT_DIRECTION_X[entry_id] = -12.0 * PostureModule::lr(fighter.module_accessor);                                 
                            PostureModule::update_rot_y_lr(fighter.module_accessor);
                            VA_OPPONENT_DIRECTION_Y[entry_id] = 0.0;
                        }
                        else if YOU_X[entry_id] < VA_OPPONENT_X[entry_id] {
                            VA_OPPONENT_DIRECTION_X[entry_id] = 12.0;                                   
                            if PostureModule::lr(fighter.module_accessor) == 1.0 {
                                PostureModule::reverse_lr(fighter.module_accessor);
                                PostureModule::update_rot_y_lr(fighter.module_accessor);
                            }
                            VA_OPPONENT_DIRECTION_Y[entry_id] = 0.0;
                        }
                        else if YOU_X[entry_id] > VA_OPPONENT_X[entry_id]{
                            VA_OPPONENT_DIRECTION_X[entry_id] = -12.0; 
                            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                                PostureModule::reverse_lr(fighter.module_accessor);
                                PostureModule::update_rot_y_lr(fighter.module_accessor);                           
                            }
                            VA_OPPONENT_DIRECTION_Y[entry_id] = 0.0;
                        }                
                    }
                    if (YOU_Y[entry_id] - VA_OPPONENT_Y[entry_id]).abs() <= 12.0 // Checks to see if you and your opponent are "close enough" in Y value to send into the air
                    && StatusModule::situation_kind(VA_OPPONENT_BOMA [entry_id] as *mut smash::app::BattleObjectModuleAccessor) == *SITUATION_KIND_GROUND {
                        VERT_EXTRA[entry_id] = 0.0; // If you and your opponent are "close enough" in vertical height, don't add any extra vertical distance.
                    }
                    else {
                        StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), true); // Sets you to airborne
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE); // Turns of Gravity 
                        VERT_EXTRA[entry_id] = 6.0; // Moves you slightly above the opponent, if they're far enough apart in Y values
                        PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                            x: 0.0,
                            y: 2.0
                        });
                    }
                    VANISH_COUNT[entry_id] += 1; 
                    CAMERA[entry_id] = true;
                }
                
                if VANISH_TIMER[entry_id] >= 0.0 { // This whole if statement is for linearly interpolating your position, instead of just teleporting behind the opponent.
                    if (YOU_Y[entry_id] - VA_OPPONENT_Y [entry_id]).abs() > 12.0 { // If yuor Y and the Opponent's Y are far enough apart, do the following:
                        StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), false); // Set yourself to be airborne
                    }
                    PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f{ // Linear Interpolation formula: Destination * t + Starting * (1.0 - t), where 0 <= t <= 1. You can't add vectors apparently, so I did this for both X and Y.
                        x: ((VA_OPPONENT_X[entry_id] + VA_OPPONENT_DIRECTION_X[entry_id]) * VANISH_TIMER[entry_id]) + YOU_X[entry_id] * (1.0 - VANISH_TIMER[entry_id]),
                        y: ((VA_OPPONENT_Y [entry_id] + VA_OPPONENT_DIRECTION_Y[entry_id] + VERT_EXTRA[entry_id]) * VANISH_TIMER[entry_id]) + YOU_Y[entry_id] * (1.0 - VANISH_TIMER[entry_id])
                    });
                }
            
                VANISH_TIMER[entry_id] += 0.1; // Increases the "t" in the interpolation formula by 0.1 every frame.


                //When the interpolation reaches 1.0 (when you've reached the destination), reset effects.
                if VANISH_TIMER[entry_id] > 1.0 {
                    VANISH_TIMER[entry_id] = 0.0; // Resets the interpolation timer.
                    DEFENDER_VANISH[entry_id] = false;
                    VANISH[entry_id] = false;
                    CAMERA[entry_id] = false;
                    disable_vanish_effects(fighter);
                }    
            }   
        }   
    }
}
//EFFECTS ON (START)
pub unsafe fn enable_vanish_effects(fighter : &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let opponent_boma = sv_battle_object::module_accessor(WHO_GOT_HIT_BOMA[entry_id]);
    let zero = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};
    let rotation = smash::phx::Vector3f {x:0.0,y:0.0,z:90.0}; 
    //JostleModule::set_status(fighter.module_accessor, false);
    //SlowModule::set_whole(fighter.module_accessor, 4, 32);
    macros::SLOW_OPPONENT(fighter, 100.0, 10.0);
    PostureModule::reverse_lr(fighter.module_accessor); 
    EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_attack_speedline"), smash::phx::Hash40::new("waist"), &zero, &rotation, 1.2, &zero, &zero, false, 0, 0, 0);
    macros::BURN_COLOR(fighter, 0.0, 0.0, 0.0, 1.0);
    VisibilityModule::set_whole(fighter.module_accessor, false);
    MotionModule::set_rate(fighter.module_accessor, 0.001);   
}
//EFFECTS OFF (END)
pub unsafe fn disable_vanish_effects(fighter : &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let opponent_boma = sv_battle_object::module_accessor(WHO_GOT_HIT_BOMA[entry_id]);
    let zero = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};
    let rotation = smash::phx::Vector3f {x:0.0,y:0.0,z:90.0};
    
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
        float(fighter);
        FLOAT[entry_id] = true;
    }
    else if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
    }

    VisibilityModule::set_whole(fighter.module_accessor, true); 
    EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_attack_speedline"), smash::phx::Hash40::new("waist"), &zero, &rotation, 1.0, &zero, &zero, false, 0, 0, 0);
    JostleModule::set_status(fighter.module_accessor, true);   
    macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);  
    MotionModule::set_rate(fighter.module_accessor, 1.0);       
    SlowModule::clear_whole(fighter.module_accessor);
}
//FLOAT
#[fighter_frame_callback]
pub fn float(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        
        //If fall and 10 frames in, while FLOAT is true:
        if status_kind == *FIGHTER_STATUS_KIND_FALL
        && FLOAT[entry_id] {  
            FLOAT_TIMER[entry_id] += 1;
            KineticModule::clear_speed_all(fighter.module_accessor);//Float
            if FLOAT_TIMER[entry_id] >= 10 {
                FLOAT[entry_id] = false;//Turn Float off
                FLOAT_TIMER[entry_id] >= 0;
            }
        }
        else {
            FLOAT[entry_id] = false;//Turn Float off
        }
    }
}
//RSETS
#[fighter_frame_callback]
pub fn resets(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        //RESSETS      
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
            VANISH[entry_id] = false;
            DEFENDER_VANISH[entry_id] = false;
        } 
        if smash::app::sv_information::is_ready_go() == false {
            VANISH_COUNT[entry_id] = 0;
        }
    }
}
// Vanish as many times as you can jump 
#[fighter_frame_callback]  
pub fn vanish_counter(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let max_jumps = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        let edgde_one_wing_max_jumps = WorkModule::get_int(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_JUMP_COUNT_MAX);

        //if LIGHTNING[entry_id] {
            if (max_jumps == 2 && VANISH_COUNT[entry_id] <2)
            || (max_jumps == 3 && VANISH_COUNT[entry_id] <3) 
            || (max_jumps == 4 && VANISH_COUNT[entry_id] <4) 
            || (max_jumps == 5 && VANISH_COUNT[entry_id] <5) 
            || (max_jumps == 6 && VANISH_COUNT[entry_id] <6)
            || (edgde_one_wing_max_jumps == 3 && VANISH_COUNT[entry_id] <3)
            {
                CAN_VANISH[entry_id] = true; 
            }
            else {
                CAN_VANISH[entry_id] = false;
            }                    
        //}  
    }
} 
pub fn install() {

    smashline::install_agent_frame_callbacks!(vanish, vanish_button, resets, float, vanish_counter);
}