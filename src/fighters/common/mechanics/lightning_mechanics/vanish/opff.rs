use super::*;

//CONDITION
#[fighter_frame_callback]
pub fn vanish_condition(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        //if CAN_VANISH {

            if ENABLE_ATTACK_CANCEL[entry_id] {
                ENABLE_CANCEL_INTO_VANISH[entry_id] = true;//Every instance of enable attack cancel opens this up
            }            
            if  (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {
                if is_cancel_into_vanish_conditions(fighter/*, true, true*/)
                { 
                    cancel_into_vanish(fighter);
                }  
                else if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_DAMAGE_FLY 
                {
                    vanish_defense(fighter); 
                }                
            }

        //}        
    }
}

//CANCEL INTO VANISH
pub unsafe fn is_cancel_into_vanish_conditions(
    fighter: &mut L2CFighterCommon/*, 
    state: bool,      
    //next_input: bool,
    prev_state: bool*/)-> bool {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        ENABLE_CANCEL_INTO_VANISH[entry_id]
        && is_after_hitlag(fighter)
        
    }
}

pub unsafe fn cancel_into_vanish(fighter : &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    attack_vanish_get_current_position(fighter);  
    
}

//DEFENSE VERSION
pub fn vanish_defense(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

        if VANISH[entry_id] == false
        {
            DEFENDER_VANISH[entry_id] = true;
            DamageModule::set_damage_lock(fighter.module_accessor, true);
            DamageModule::set_no_reaction_no_effect(fighter.module_accessor, true); 
            HitModule::set_hit_stop_mul(fighter.module_accessor, 0.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);

        }
        else if DEFENDER_VANISH[entry_id] 
        && VANISH[entry_id] == false {
            DamageModule::set_damage_lock(fighter.module_accessor, false);
            DamageModule::set_no_reaction_no_effect(fighter.module_accessor, false);
            HitModule::set_hit_stop_mul(fighter.module_accessor, 1.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
            DEFENDER_VANISH[entry_id] = false;
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
    
    CANCEL_INTO_VANISH[entry_id] = false;
    DIRECT_HIT[entry_id] = false;  
    PROJECTILE_HIT[entry_id] = false;  
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
        FLOAT[entry_id] = true;
    }
    else if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
    }
    ControlModule::clear_command(fighter.module_accessor, false);
    VisibilityModule::set_whole(fighter.module_accessor, true); 
    EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_attack_speedline"), smash::phx::Hash40::new("waist"), &zero, &rotation, 1.0, &zero, &zero, false, 0, 0, 0);
    JostleModule::set_status(fighter.module_accessor, true);   
    macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);  
    MotionModule::set_rate(fighter.module_accessor, 1.0);       
    SlowModule::clear_whole(fighter.module_accessor);
}




#[weapon_frame_callback]
pub fn vanish_attack_weapon(weapon : &mut L2CFighterBase) {
    unsafe {
        let entry_id = WorkModule::get_int(weapon.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        if AttackModule::is_attack_occur(weapon.module_accessor){
            PROJECTILE_HIT[entry_id] = true;
        }
    }
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

                    //CHECK IF STAGE IS IN BATTLEFIELD/OMEGA FORM, ELSE FIND BOUNDS OF EACH STAGE. PUT THAT IN A FUNCTION.
                    //OR CHECK FOR THE BOUNDS OF A BATTLEFIELD/OMEGA FORM STAGE.

                    /*if VA_OPPONENT_X[entry_id] <= 120.0
                    && VA_OPPONENT_X[entry_id] >= -150.0
                    && VA_OPPONENT_Y[entry_id] <= 150.0 
                    && VA_OPPONENT_Y[entry_id] >= 70.0 {// IN THE BOUNDS OF A BATTLEFIELD/OMEGA FORM STAGE. 

                    }*/
                    if stick_manipulation && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {//Manipulate position using a formula converting stickc_degrees to the point of circumference (12x/12y inversion unit radius)

                        VA_OPPONENT_DIRECTION_X[entry_id] = (dir_x * -12.0); //(stick driection * how far from the opponent's destination)
                        VA_OPPONENT_DIRECTION_Y[entry_id] = (dir_y * -12.0);                  
                        
                    }
                    else {//If no stick direction, default to facing behind the opponent
                        
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
                    if (YOU_Y[entry_id] - VA_OPPONENT_Y[entry_id]).abs() <= 12.0 // Checks to see if you and your opponent are "close enough" in Y value to send into the air. SINCE CHARACTERS VARY IN SIZE, this will change based on their actual height in units.
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
                    VANISH_HEIGHT[entry_id] = 0.0;
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

    smashline::install_agent_frame_callbacks!(
        vanish, 
        float,
        vanish_counter, 
        vanish_condition
    );
}