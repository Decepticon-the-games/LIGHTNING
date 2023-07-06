use super::*;

#[fighter_frame_callback]
pub fn attack_cancels(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

//ATTACK CANCELS
        if ATTACK_CANCEL[entry_id] 
        && ! (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STATUS) || status_kind == *FIGHTER_STATUS_KIND_FINAL) {// This line is not needed after all characters get the acmd rendition
            
            //If an attack occurs, cancel out of it (after hitlag)
            if (AttackModule::is_attack_occur(fighter.module_accessor) 
            && SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) == 0)
            { 
                CancelModule::enable_cancel(fighter.module_accessor);
                ENABLE_ATTACK_CANCEL[entry_id] = false;
                ATTACK_CANCEL[entry_id] = false; 
            }  
        }
        else  {
            ATTACK_CANCEL[entry_id] = false;
        } 
    }
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(attack_cancels);
}