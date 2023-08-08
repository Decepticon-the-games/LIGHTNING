use super::*;
use crate::fighters::common::mechanics::cancels::attack_cancels::{ENABLE_ATTACK_CANCEL,ENABLE_MULTIHIT_CANCEL};
use crate::fighters::common::mechanics::lightning_mechanics::lightning_mode::LIGHTNING;





#[fighter_frame( agent = FIGHTER_KIND_PIKACHU )]

    pub fn pikachu_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            //let status_kind = StatusModule::status_kind(module_accessor);
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            //let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);


        }
    }

pub fn install() {
    smashline::install_agent_frames!(pikachu_opff);

}