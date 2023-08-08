use super::*;

#[fighter_frame_callback]
pub fn common_combo_system(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;   
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 0);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
  
    //If on successful hit, If the prev successful move is the same as the current successful move, disable cancel to that move until another status is executed.
        
        if is_attack_cancel(
            fighter/*, 
            status_kind == *FIGHTER_STATUS_KIND_ATTACK, 
            next_input, 
            !prev_status_kind == *FIGHTER_STATUS_KIND_ATTACK,*/
        ) {
            if ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK && prev_status_kind == *FIGHTER_STATUS_KIND_ATTACK && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0) 
            {//jab string doesn't go into itself
                cancel_on_hit(fighter);
            }
        }
    }
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(common_combo_system);
}