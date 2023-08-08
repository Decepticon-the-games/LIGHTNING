use super::*;

//These attack_cancel functions get the state your in (status_kind, flag, or motion_kind), your next button, the previous state you were in.

//ATTACK_CANCELS (attacks cancellable at the last hitbox only)
pub unsafe fn is_attack_cancel(
    fighter: &mut L2CAgentBase/*, 
    state: bool,      
    next_input: bool,
    state2: bool*/) -> bool{
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 0);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
    

        //ENABLE_ATTACK CANCELS
        ENABLE_ATTACK_CANCEL[entry_id]
        && is_after_hitlag(fighter)
        //&& (state && state2 /*&& next_input*/)
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

//If an attack occurs, cancel out of attack (after hitlag)
pub unsafe fn cancel_on_hit(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    
    if is_after_hitlag(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor); 
    }
}

