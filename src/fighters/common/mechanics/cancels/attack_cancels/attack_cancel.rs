use super::*;

#[fighter_frame_callback]
pub fn enable_attack_cancels(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let jab_combo_flags = (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)); 
        let final_smash = (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STATUS) || status_kind == *FIGHTER_STATUS_KIND_FINAL);
//ATTACK CANCELS
        if ENABLE_ATTACK_CANCEL[entry_id] {
            if ! final_smash // This line is not needed after all characters get the acmd rendition
            || (jab_combo_flags && ! (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0) {
                //If an attack occurs, cancel out of it (after hitlag)
                /*if (AttackModule::is_attack_occur(fighter.module_accessor) 
                && SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) == 0)
                { 
                    CancelModule::enable_cancel(fighter.module_accessor);
                    
                    ENABLE_ATTACK_CANCEL[entry_id] = false;
                    //ATTACK_CANCEL[entry_id] = false; 
                    CANCEL_IN_NEUTRAL[entry_id] = false;                
                }*/
                attack_cancel(fighter);
                //ENABLE_ATTACK_CANCEL[entry_id] = false;//WHEN U COME BACK HOME  CHECK THISSSSSS

                
                
            }           
        }
    }
}

//#[fighter_frame_callback]
pub fn attack_cancel(fighter: &mut L2CFighterCommon) {
unsafe {
        //let fighter : &mut L2CFighterCommon = todo!();
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

//ATTACK CANCELS
            //If an attack occurs, cancel out of it (after hitlag)
            if (AttackModule::is_attack_occur(fighter.module_accessor) 
            && SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) == 0)
            { 
                CancelModule::enable_cancel(fighter.module_accessor);
                ENABLE_ATTACK_CANCEL[entry_id] = false;
                CANCEL_IN_NEUTRAL[entry_id] = false;                
            }
}
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(
        //attack_cancel,
        enable_attack_cancels
    );
}