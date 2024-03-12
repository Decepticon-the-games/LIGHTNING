use super::*;

#[fighter_frame_callback]
pub fn resets_falses(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
            ATTACK_CANCEL_COUNT[entry_id] = 0;
            ENABLE_ATTACK_CANCEL[entry_id] = false;
            ATTACK_CANCEL[entry_id] = false;  
            MULTIHIT_COUNT[entry_id] = 0;
            ENABLE_MULTIHIT_CANCEL[entry_id] = false;
        } 
        /*
        if ENABLE_ATTACK_CANCEL[entry_id] {

            ENABLE_MULTIHIT_CANCEL[entry_id] = false;

            //If interrupted, turn off
            if CancelModule::is_enable_cancel(fighter.module_accessor)
            || StopModule::is_damage(fighter.module_accessor)//If interrupted by a collision/taken damage
            || status_kind == *FIGHTER_STATUS_KIND_DAMAGE
            || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR
            {
                ENABLE_ATTACK_CANCEL[entry_id] = false;                     
            }
        } 
        else if CANCEL_IN_NEUTRAL[entry_id]  {//Once Cancel in neutral activates, turn off all atack cancelling.
            ENABLE_ATTACK_CANCEL[entry_id] = false; 
            ATTACK_CANCEL[entry_id] = false;
        }
        */
    }
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(resets_falses);
}