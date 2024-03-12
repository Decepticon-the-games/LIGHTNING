use super::*;

//AIRSTEP CANCELLING ON HIT IN THE AIR
#[fighter_frame_callback]
pub fn airstep_setup(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0); 
        let status_kind = StatusModule::status_kind(fighter.module_accessor);

        if ATTACK_CANCEL[entry_id] {
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0
            && ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 
            /*|| (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0*/)
            {
                AIRSTEP_BUTTON[entry_id] = true;
            }
            else {
               AIRSTEP_BUTTON[entry_id] = false; 
            }
        }
        if AIRSTEP_BUTTON[entry_id] && is_after_hitlag(fighter) {

            AIRSTEP[entry_id] = true;
            AIRSTEP_BUTTON[entry_id] = false;

        }  
    }
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(airstep_setup);
}
