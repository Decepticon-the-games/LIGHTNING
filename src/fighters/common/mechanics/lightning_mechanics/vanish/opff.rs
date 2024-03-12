use super::*;

//CONDITION
#[fighter_frame_callback]
pub fn vanish_condition(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let opponent_boma = sv_battle_object::module_accessor(WHO_GOT_HIT_BOMA[entry_id]);

        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        let speed = KineticModule::get_sum_speed_x(opponent_boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_DAMAGE);

        //MORE THAN ONE PLAYER SHOULD NOT BE ABLE TO DO THIS AT THE SAME TIME!!!!!
        if entry_id < 1 { 
            //println!("speed: {}", speed);
        }

        if entry_id < 8 {

            //if CAN_VANISH[entry_id] {

                /*//-----------------------testing only---------------------------------//
                vanish_defense(fighter); 
                if attack_vanish_conditions(fighter) { 
                    attack_vanish_get_current_position(fighter); 
                    FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_VANISH[entry_id] = false; 
                }  
                //---------------------------------------------------------------------//
                */

                if attack_vanish_conditions(fighter) 
                || defense_vanish_conditions(fighter)
                {
                    FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_VANISH[entry_id] = true;//The instance of these conditions opens this up. (IN TRAINING MODE, should indicate when you can vanish somehow)
                }     
                else if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                    FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_VANISH[entry_id] = false;
                }
                
                if FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_VANISH[entry_id] {
 
                    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0
                    && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0
                    {
                        if attack_vanish_conditions(fighter) {
                           //VANISH_BUFFER[entry_id] = true; 
                            attack_vanish_get_current_position(fighter); 
                            FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_VANISH[entry_id] = false; 
                        }   
                    }   
                    else if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                        if defense_vanish_conditions(fighter) {
                           vanish_defense(fighter); 
                           FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_VANISH[entry_id] = false; 
                        } 
                    }
                    else {
                        FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_VANISH[entry_id] = false; 
                    }
                }
                /*if VANISH_BUFFER[entry_id] {
                    if (ATTACK_CANCEL[entry_id] && is_after_hitlag(fighter))
                    || is_after_hitlag(fighter) // So that u can press it during hitlag or after hitlag
                    {
                        attack_vanish_get_current_position(fighter); 
                        VANISH_BUFFER[entry_id] = false;
                    }                    
                }*/
            //}  
        }      
    }
}

//CANCEL INTO VANISH
pub unsafe fn attack_vanish_conditions(fighter: &mut L2CFighterCommon) -> bool {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let opponent_boma = sv_battle_object::module_accessor(WHO_GOT_HIT_BOMA[entry_id]);

        ATTACK_CANCEL[entry_id] 
        && DamageModule::reaction(opponent_boma, 0) < 110.0 && DamageModule::reaction(opponent_boma, 0) != 0.0
        && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR 
    }
}

pub unsafe fn defense_vanish_conditions(fighter : &mut L2CFighterCommon) -> bool {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_DAMAGE_FLY 
    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR
}

//DEFENSE VERSION
pub fn vanish_defense(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

        //if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR
        {
            if DEFENDER_VANISH[entry_id] == false && FIGHTER_STATUS_KIND_VANISH[entry_id] == false 
            {
                DamageModule::set_damage_lock(fighter.module_accessor, true);
                DamageModule::set_no_reaction_no_effect(fighter.module_accessor, true); 
                HitModule::set_hit_stop_mul(fighter.module_accessor, 0.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0); 
                DEFENDER_VANISH[entry_id] = true;                
            }
            else {
                DamageModule::set_damage_lock(fighter.module_accessor, false);
                DamageModule::set_no_reaction_no_effect(fighter.module_accessor, false);
                HitModule::set_hit_stop_mul(fighter.module_accessor, 1.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
            }
        }
    }
}

//GET OPPONENT POSITION
pub unsafe fn attack_vanish_get_current_position(fighter : &mut L2CFighterCommon) { //RECALL PLAYER ID VARIABLE, and store their position
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let opponent_boma = sv_battle_object::module_accessor(WHO_GOT_HIT_BOMA[entry_id]);
    let speed = KineticModule::get_sum_speed_x(opponent_boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_DAMAGE);

    VA_OPPONENT_X[entry_id] = PostureModule::pos_x(opponent_boma);
    VA_OPPONENT_Y[entry_id] = PostureModule::pos_y(opponent_boma);
    VA_OPPONENT_BOMA[entry_id] = (&mut *opponent_boma as *mut BattleObjectModuleAccessor) as u64;
    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
    if utility::get_category(&mut *opponent_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        JostleModule::set_status(&mut *opponent_boma, false);
    }
    if (YOU_Y[entry_id] - VA_OPPONENT_Y[entry_id]).abs() <= 12.0 
    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND //If distance is less than 12.0 || Opponent KB speed is over 80...
    && speed >= 3.0 //opponent knockback speed. Smash5's kn speed always slows down under 3 exponentially, so this is a good spot to be able to connect moves.
    {
        FIGHTER_STATUS_KIND_VANISH[entry_id] = false;
    }
    else {
        FIGHTER_STATUS_KIND_VANISH[entry_id] = true; //...Vanish will be performed.    
    }
}

//EFFECTS ON (START)
pub fn enable_vanish_effects(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let opponent_boma = sv_battle_object::module_accessor(WHO_GOT_HIT_BOMA[entry_id]);
        let zero = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};
        let rotation = smash::phx::Vector3f {x:0.0,y:0.0,z:90.0}; 

        DamageModule::set_damage_lock(fighter.module_accessor, true);
        DamageModule::set_no_reaction_no_effect(fighter.module_accessor, true); 
        HitModule::set_hit_stop_mul(fighter.module_accessor, 0.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0); 
        macros::SLOW_OPPONENT(fighter, 999.0, 10.0);
        EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_attack_speedline"), smash::phx::Hash40::new("waist"), &zero, &rotation, 1.2, &zero, &zero, false, 0, 0, 0);
        macros::LAST_EFFECT_SET_COLOR(fighter,0.0, 0.0, 0.0,);
        macros::BURN_COLOR(fighter, 0.0, 0.0, 0.0, 1.0);
        VisibilityModule::set_whole(fighter.module_accessor, false);
        MotionModule::set_rate(fighter.module_accessor, 0.001);              
    }
}

//EFFECTS OFF (END)
pub fn disable_vanish_effects(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let opponent_boma = sv_battle_object::module_accessor(WHO_GOT_HIT_BOMA[entry_id]);
        let zero = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};
        let rotation = smash::phx::Vector3f {x:0.0,y:0.0,z:90.0};
        
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            //FLOAT[entry_id] = true;
        }
        else if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
        }
        DamageModule::set_damage_lock(fighter.module_accessor, false);
        DamageModule::set_no_reaction_no_effect(fighter.module_accessor, false);
        HitModule::set_hit_stop_mul(fighter.module_accessor, 1.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
        ControlModule::clear_command(fighter.module_accessor, false);
        VisibilityModule::set_whole(fighter.module_accessor, true); 
        EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_attack_speedline"), smash::phx::Hash40::new("waist"), &zero, &rotation, 1.0, &zero, &zero, false, 0, 0, 0);
        macros::LAST_EFFECT_SET_COLOR(fighter,0.0, 0.0, 0.0,);
        macros::BURN_COLOR(fighter, 0.0, 0.0, 0.0, 1.0);
        JostleModule::set_status(fighter.module_accessor, true);   
        macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);  
        MotionModule::set_rate(fighter.module_accessor, 1.0);   
        macros::CAM_ZOOM_OUT(fighter); // Resets the camera.
        macros::CANCEL_FILL_SCREEN(fighter, 0, 5); // Clears out the background screen darken effect.
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), true, true);
        if FIGHTER_INSTANCE_WORK_ID_FLAG_CRIMSON_CANCEL[entry_id] {//So crimson's slowdown doesn't get disrupted
           macros::SLOW_OPPONENT(fighter, 5.0, 120.0);  
        }    
        else {
            SlowModule::clear_whole(fighter.module_accessor); 
        } 
    }
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
        let opponent_boma = sv_battle_object::module_accessor(WHO_GOT_HIT_BOMA[entry_id]);
        let dir_x = ControlModule::get_stick_x(fighter.module_accessor);
        let dir_y = ControlModule::get_stick_y(fighter.module_accessor);
        let stick_degrees = dir_y.atan2(dir_x).to_degrees();         
        let stick_manipulation = (dir_x >= 0.8 || dir_x <= -0.8 || dir_y >= 0.8 || dir_y <= -0.8 );

        
        if entry_id < 8 {
            
            if FIGHTER_STATUS_KIND_VANISH[entry_id] {
                 
                if CAMERA[entry_id] == false {
                    enable_vanish_effects(fighter);

                    YOU_X[entry_id] = PostureModule::pos_x(fighter.module_accessor); // Gets your position
                    YOU_Y[entry_id] = PostureModule::pos_y(fighter.module_accessor);
                    
                    VA_OPPONENT_X[entry_id] = PostureModule::pos_x(opponent_boma);
                    VA_OPPONENT_Y[entry_id] = PostureModule::pos_y(opponent_boma);//Get's the opponent's position, updating every frame??
                    
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
                    DEFENDER_VANISH[entry_id] = false;
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
                    //VANISH_HEIGHT[entry_id] = 0.0;
                    VANISH_TIMER[entry_id] = 0.0; // Resets the interpolation timer.
                    CAMERA[entry_id] = false;
                    disable_vanish_effects(fighter);
                    FIGHTER_STATUS_KIND_VANISH[entry_id] = false;
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
 
// Vanish as many times as you can jump. Resets on ground
#[fighter_frame_callback]  
pub fn vanish_counter(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let max_jumps = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        let edgde_one_wing_max_jumps = WorkModule::get_int(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_JUMP_COUNT_MAX);

        //if IS_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING[entry_id] {
            if (max_jumps == 2 && VANISH_COUNT[entry_id] >= 2)
            || (max_jumps == 3 && VANISH_COUNT[entry_id] >= 3) 
            || (max_jumps == 4 && VANISH_COUNT[entry_id] >= 4) 
            || (max_jumps == 5 && VANISH_COUNT[entry_id] >= 5) 
            || (max_jumps == 6 && VANISH_COUNT[entry_id] >= 6)
            || (edgde_one_wing_max_jumps == 3 && VANISH_COUNT[entry_id] >= 3)
            {
                FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_VANISH[entry_id] = false;
            }                    
        //}  
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            VANISH_COUNT[entry_id] == 0;
        }
    }
}


pub fn install() {

    smashline::install_agent_frame_callbacks!(
        vanish, 
        //float,
        vanish_counter, 
        vanish_condition
    );
}