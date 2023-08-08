use super::*;

#[fighter_frame( agent = FIGHTER_KIND_MARIO )]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
        let frame = MotionModule::frame(fighter.module_accessor);
        ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
            
        //In Lightning...
        if LIGHTNING[entry_id] {
            //Cancel Dair only right before last hit
            let next_input = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0;
            multihit_cancel(fighter, 0, 0, smash::hash40("attack_air_lw"), next_input, 0, 0, smash::hash40("attack_air_lw")); 
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(once_per_fighter_frame);
}