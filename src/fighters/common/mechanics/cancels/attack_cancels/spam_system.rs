use super::*;

#[fighter_frame_callback]
pub fn spam_system(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;   
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        let movement_cancel = (ControlModule::is_enable_flick_jump(fighter.module_accessor) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0) 
        || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 
        || (((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0) && situation_kind == *SITUATION_KIND_GROUND) 
        || (((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0) && situation_kind == *SITUATION_KIND_AIR);




        
        
        if ENABLE_ATTACK_CANCEL[entry_id]
        {

//Prevent Jabs from cancelling into themselves
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
                    ATTACK_CANCEL[entry_id] = false; 
                    ENABLE_ATTACK_CANCEL[entry_id] =false;
                }
            }

// this instance prevents cancelling more than a certain amount without first moving
            if AttackModule::is_attack_occur(fighter.module_accessor) {
                if ATTACK_CANCEL_COUNTER[entry_id] == false {
                    ATTACK_CANCEL_COUNT[entry_id] +=1;
                    ATTACK_CANCEL_COUNTER[entry_id] = true;
                    //ENABLE_ATTACK_CANCEL[entry_id] = false; 
                }
                //the amount of times u can cancel in a row
                if ATTACK_CANCEL_COUNT[entry_id] > 2 {
                    ATTACK_CANCEL_COUNT[entry_id] = 3;
                    ATTACK_CANCEL[entry_id] = false;       
                } 
                else {
                    ATTACK_CANCEL[entry_id] = true;
                }            
            }
            else {
                ATTACK_CANCEL_COUNTER[entry_id] = false; 
            }
            //moovement options, always reset attack cancel count
            if movement_cancel {
                ATTACK_CANCEL_COUNT[entry_id] = 0; 
            }  
            //ENABLE_ATTACK_CANCEL[entry_id] = false;
        } 
    }
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(spam_system);
}