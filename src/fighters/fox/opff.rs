use {
    smash::{
        lua2cpp::{L2CAgentBase,L2CFighterCommon},
        phx::Hash40,
        hash40,
        app::{lua_bind::*, sv_animcmd::*,*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};
use crate::fighters::common::mechanics::attack_cancels::ENABLE_ATTACK_CANCEL;






#[fighter_frame( agent = FIGHTER_KIND_FOX )]

    pub fn fox_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            let status_kind = StatusModule::status_kind(module_accessor);
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);

//Enable cancel 

    if AttackModule::is_attack_occur(fighter.module_accessor) {

            ENABLE_ATTACK_CANCEL[entry_id] = true;
    }
            


//Fast fall laser
            if (motion_kind == smash::hash40("special_n_loop")
            || motion_kind == smash::hash40("special_air_n_loop"))
            && (ControlModule::get_command_flag_cat(module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0 {
                WorkModule::set_flag(module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }

//jump cancel shine (multishine)
            if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT 
            && frame > 1.0 {
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                    //CancelModule::enable_cancel(fighter.module_accessor);
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                }
            }

        }
    }

pub fn install() {
    smashline::install_agent_frames!(fox_opff);

}