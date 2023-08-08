use super::*;


#[fighter_frame( agent = FIGHTER_KIND_BAYONETTA )]

    pub fn bayonetta_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            //let status_kind = StatusModule::status_kind(module_accessor);
            //let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            //let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
            let fair_combo_flag =  WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO);
            if SearchModule::is_inflict(fighter.module_accessor) && SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_NORMAL) == 0 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }

        }
    }

pub fn install() {
    smashline::install_agent_frames!(bayonetta_opff);

}