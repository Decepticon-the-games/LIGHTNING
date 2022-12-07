use super::*;

pub static mut ATTACK_CANCEL : [bool; 8] = [false; 8];

#[fighter_frame_callback]
pub fn attack_cancels(fighter : &mut L2CFighterCommon) {
    unsafe {

        //ATTACK CANCELS
        if ATTACK_CANCEL[entry_id] 
        && ! (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STATUS) || status_kind == *FIGHTER_STATUS_KIND_FINAL) {
            
            if AttackModule::is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor) { 
                CancelModule::enable_cancel(fighter.module_accessor);
                
            }
            //VANISH
            if AttackModule::is_attack_occur(fighter.module_accessor) || PROJECTILE_HIT[entry_id] {
                CANCEL_INTO_VANISH[entry_id] = true;
            }                          
            else{
                CANCEL_INTO_VANISH[entry_id] = false;
            }
            ATTACK_CANCEL[entry_id] = false;         
        }
        else  {
            ATTACK_CANCEL[entry_id] = false;
        } 
    }
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(attack_cancels);
}