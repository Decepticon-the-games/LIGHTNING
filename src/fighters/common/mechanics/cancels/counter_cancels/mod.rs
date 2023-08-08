use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;
use crate::fighters::common::mechanics::cancels::attack_cancels::ENABLE_ATTACK_CANCEL;
use crate::fighters::common::mechanics::cancels::motioncancels::CANCEL_IN_NEUTRAL;

pub static mut COUNTER_CANCEL : [bool; 8] = [false; 8];

#[fighter_frame_callback]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let fighter_kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
        //let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

        //println!("cc: {}", COUNTER_CANCEL[entry_id]);
        

        if COUNTER_CANCEL[entry_id] {
            if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) {
                CancelModule::enable_cancel(fighter.module_accessor);
                COUNTER_CANCEL[entry_id] = false;
            }
            else if fighter_kind == *FIGHTER_KIND_SHULK && ! ((cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0) || (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0)) {
                CancelModule::enable_cancel(fighter.module_accessor);
                COUNTER_CANCEL[entry_id] = false;
            }
            else if CANCEL_IN_NEUTRAL[entry_id] || ENABLE_ATTACK_CANCEL[entry_id] {//TRY IS_ATTACK_OCCUR AND ELSE INPUT
                COUNTER_CANCEL[entry_id] = false;
            }
        }        
    }                                      
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(once_per_fighter_frame);
}