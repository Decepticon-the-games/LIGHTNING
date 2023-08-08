use super::*;

//CANCEL INTO VANISH
pub unsafe fn cancel_into_vanish(
    fighter: &mut L2CAgentBase, 
    status: i32,     
    flag: i32, 
    motion: u64, 
    prev_status: i32, 
    off_flag: i32) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 0);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

        let prev = (
        StatusModule::prev_status_kind(fighter.module_accessor, 0) == prev_status
        || !WorkModule::is_flag(fighter.module_accessor, off_flag)
        );

        let next_input = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0;

        if ENABLE_ATTACK_CANCEL[entry_id] 
        && is_after_hitlag(fighter)
        {
            CANCEL_INTO_VANISH[entry_id] = true; 
        }       

        if CANCEL_INTO_VANISH[entry_id]
        {
            //if next_input {
                //vanish_attack(fighter);  
            //}
        }       

        if PROJECTILE_HIT[entry_id] {
            //if next_input {
                //vanish_attack(fighter);  
            //}
        }
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_DAMAGE_FLY {
            if next_input {
                vanish_defense(fighter); 
            }         
        }
    }
}
//ATTACK VERSION
pub unsafe fn vanish_attack(fighter : &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if is_after_hitlag(fighter)
    {
        attack_vanish_get_current_position(fighter);
    }            
}
//DEFENSE VERSION
pub fn vanish_defense(fighter : &mut L2CAgentBase) {
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
pub unsafe fn attack_vanish_get_current_position(fighter : &mut L2CAgentBase) { //RECALL PLAYER ID VARIABLE, and store their position
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
pub unsafe fn enable_vanish_effects(fighter : &mut L2CAgentBase) {
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
pub unsafe fn disable_vanish_effects(fighter : &mut L2CAgentBase) {
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
    CancelModule::enable_cancel(fighter.module_accessor);
    VisibilityModule::set_whole(fighter.module_accessor, true); 
    EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_attack_speedline"), smash::phx::Hash40::new("waist"), &zero, &rotation, 1.0, &zero, &zero, false, 0, 0, 0);
    JostleModule::set_status(fighter.module_accessor, true);   
    macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);  
    MotionModule::set_rate(fighter.module_accessor, 1.0);       
    SlowModule::clear_whole(fighter.module_accessor);
}