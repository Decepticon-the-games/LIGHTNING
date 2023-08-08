use super::*;

#[fighter_frame( agent = FIGHTER_KIND_EDGE )]
pub fn edge_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        ////let situation_kind = StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
    }                                      
}

pub fn install() {
    smashline::install_agent_frames!(edge_opff);

}