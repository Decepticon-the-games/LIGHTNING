use {
    smash::{
        lua2cpp::{L2CAgentBase,L2CFighterCommon,L2CFighterBase},
        phx::Hash40,
        hash40,
        app::{lua_bind::*, sv_animcmd::*,*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};
use crate::fighters::common::mechanics::cancels::attack_cancels::ENABLE_ATTACK_CANCEL;
pub static mut DH_CANCEL : [bool; 8] = [false; 8];
pub static mut KAMUI_DH_CANCEL : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_KAMUI )]

    pub fn kamui_dragonhand_cancel(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

            if KAMUI_DH_CANCEL[entry_id] && SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) == 0 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
    }

#[weapon_frame( agent = WEAPON_KIND_KAMUI_DRAGONHAND )]

    pub fn kamui_dragonhand_opff(weapon : &mut L2CFighterBase) {
        unsafe {
            let entry_id = WorkModule::get_int(weapon.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            //let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            //let status_kind = StatusModule::status_kind(module_accessor);
            //let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            //let frame = MotionModule::frame(fighter.module_accessor);
            ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            //let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
        
            if DH_CANCEL[entry_id] {
                if AttackModule::is_attack_occur(weapon.module_accessor) {
                    //kamui_dragonhand_cancel(fighter);
                    KAMUI_DH_CANCEL[entry_id] = true;
                    DH_CANCEL[entry_id] = false;
                }
            }

//New subtititle for any other code, if not applicable just delete the lines

        }
    }


pub fn install() {
    smashline::install_agent_frames!(
        //kamui_dragonhand_opff, 
        //kamui_dragonhand_cancel
    );

}