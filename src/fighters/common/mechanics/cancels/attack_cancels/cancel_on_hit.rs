use super::*;

//These attack_cancel functions get the state your in (status_kind, flag, or motion_kind), your next button, the previous state you were in.

//ATTACK_CANCELS (attacks cancellable at the last hitbox only)
pub unsafe fn enable_attack_cancel(fighter : &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if ! AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
        ENABLE_ATTACK_CANCEL[entry_id] = true;
    }
}

pub fn is_combo_flags(fighter : &mut L2CFighterCommon) -> bool { 
    unsafe {
        WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_INPUT)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_ENABLE_COMBO)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_ENABLE_COMBO)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    }
}

#[fighter_frame_callback]
pub fn attack_cancel(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;   
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 0);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        let frame = MotionModule::frame(fighter.module_accessor);
        let combo_flags = (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_INPUT)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_ENABLE_COMBO)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_ENABLE_COMBO)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_COMBO));
    //If on successful hit, If the prev successful move is the same as the current successful move, disable cancel to that move until another status is executed.
        
        if ENABLE_ATTACK_CANCEL[entry_id] {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {

                ATTACK_CANCEL[entry_id] = true; 
                ENABLE_ATTACK_CANCEL[entry_id] = false;
                //WorkModule::unable_transition_term_group_ex(fighter.module_accessor,)
            }
            else if CANCEL_IN_NEUTRAL[entry_id] {
                ENABLE_ATTACK_CANCEL[entry_id] = false;
            }
        }
        if ATTACK_CANCEL[entry_id] {
            cancel_on_hit(fighter);
            //Extend buffer frames each frame of hitlag
            ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
        }

        //Enable whiff cancel on a landindg aerial/special fall landing
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR && ATTACK_CANCEL[entry_id] {
            LANDING_ATTACK_AIR_CANCEL[entry_id] = true;
        }
        if LANDING_ATTACK_AIR_CANCEL[entry_id] {
            if (status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR || status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL) && frame >1.0 {
                CancelModule::enable_cancel(fighter.module_accessor);
                LANDING_ATTACK_AIR_CANCEL[entry_id] = false;
            }
        }  
    }
}

//MULTIHIT_CANCELS (multihit attacks)
//___________________
//(canceling at a specific point in a multihit move)
pub unsafe fn multihit_cancel(
    fighter: &mut L2CAgentBase, 
    status: i32, 
    flag: i32,
    motion: u64,
    next_input: bool, 
    status_reset: i32,
    flag_reset: i32, 
    motion_reset: u64) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

    let is_condition = (
    StatusModule::status_kind(fighter.module_accessor) == status
    || WorkModule::is_flag(fighter.module_accessor, flag)
    || MotionModule::motion_kind(fighter.module_accessor) == motion
    );

    let is_reset = (
    StatusModule::status_kind(fighter.module_accessor) != status_reset
    || !WorkModule::is_flag(fighter.module_accessor, flag_reset)
    || MotionModule::motion_kind(fighter.module_accessor) != motion_reset
    );

    if ENABLE_MULTIHIT_CANCEL[entry_id] {
        if is_condition {
            if next_input {
                cancel_on_hit(fighter);  
                ENABLE_MULTIHIT_CANCEL[entry_id] = false;                 
            }   
        }
        else {
            ENABLE_MULTIHIT_CANCEL[entry_id] = false;
        }
    }
}
//(canceling after landing a ceertain amount of hits in a multihit move)
pub unsafe fn multihit_counter(
    fighter: &mut L2CAgentBase, 
    status: i32,     
    flag: i32, 
    motion: u64, 
    multihitcount: i32, 
    next_input: bool, 
    status_reset: i32, 
    flag_reset: i32, 
    motion_reset: u64) {

    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

    let is_condition = (
    StatusModule::status_kind(fighter.module_accessor) == status
    || WorkModule::is_flag(fighter.module_accessor, flag)
    || MotionModule::motion_kind(fighter.module_accessor) == motion
    );

    let is_reset = (
    StatusModule::status_kind(fighter.module_accessor) != status_reset
    || !WorkModule::is_flag(fighter.module_accessor, flag_reset)
    || MotionModule::motion_kind(fighter.module_accessor) != motion
    );
    
    

    if ENABLE_MULTIHIT_CANCEL[entry_id] {
        //Multihit cancels after a certain amount of successful hits
        if is_condition {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
                if MULTIHIT[entry_id] == false {
                    MULTIHIT_COUNT[entry_id] +=1;
                    MULTIHIT[entry_id] = true; 
                }         
            }
            else {
                MULTIHIT[entry_id] = false;
            }  
        
            if MULTIHIT_COUNT[entry_id] == multihitcount { //how many hits
                if next_input {
                    cancel_on_hit(fighter);   
                    MULTIHIT_COUNT[entry_id] = 0;
                }
            }
            else {
                ENABLE_MULTIHIT_CANCEL[entry_id] = false; // if u miss the count, you can't cancel
            }
        }  
        else if is_reset {
            MULTIHIT_COUNT[entry_id] = 0;
        }        
    }        

}

//Custom if after hitlag function, only true if is_attack_occur aside from hitlag.
pub unsafe fn is_after_hitlag(fighter: &mut L2CAgentBase) -> bool {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    AttackModule::is_attack_occur(fighter.module_accessor)
    && ! AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
    && ! (SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) > 0 || StopModule::is_stop(fighter.module_accessor))
}
pub unsafe fn is_after_hitlag_weapon(weapon: &mut L2CAgentBase) -> bool {
    let entry_id = WorkModule::get_int(weapon.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    AttackModule::is_attack_occur(weapon.module_accessor)
    && ! AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_ALL)
    && ! (SlowModule::frame(weapon.module_accessor, *FIGHTER_SLOW_KIND_HIT) > 0 || StopModule::is_stop(weapon.module_accessor))
}

//If an attack occurs, cancel out of attack (after hitlag)
pub unsafe fn cancel_on_hit(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

    if ATTACK_CANCEL[entry_id]
    && is_after_hitlag(fighter) {
        ENABLE_ATTACK_CANCEL[entry_id] = false;
        ATTACK_CANCEL[entry_id] = false;
        if ! (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0 {// NO CANCELING INTO WALK
           CancelModule::enable_cancel(fighter.module_accessor); 
        }
        
        //Enable fast fall on hit if in the air
        //if (ControlModule::get_command_flag_cat(fighter.module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 {
        //    WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        //}
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(attack_cancel);
}