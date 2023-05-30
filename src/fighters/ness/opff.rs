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
use crate::fighters::common::mechanics::attack_cancels::ENABLE_ATTACK_CANCEL;
use crate::fighters::common::mechanics::motioncancels::{CANCEL_IN_NEUTRAL, DISABLE_CANCEL_IN_NEUTRAL, SIDE_SPECIAL_COUNT, SIDE_SPECIAL_COUNTER};
pub static mut NO_PKFIRE : [bool; 8] = [false; 8];
#[fighter_frame( agent = FIGHTER_KIND_NESS )]

    pub fn ness_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            let status_kind = StatusModule::status_kind(module_accessor);
            //let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            //let frame = MotionModule::frame(fighter.module_accessor);
            ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
            
        }
    }
#[weapon_frame( agent = WEAPON_KIND_NESS_PK_FIRE )]
    pub fn ness_pk_fire_opff(weapon : &mut L2CFighterBase) {
        unsafe {

            let entry_id = WorkModule::get_int(weapon.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let status_kind = StatusModule::status_kind(weapon.module_accessor);

            if status_kind == *WEAPON_NESS_PK_FIRE_STATUS_KIND_PILLAR
            {
                NO_PKFIRE[entry_id] = true;
            }
            else {
                NO_PKFIRE[entry_id] = false;
            }
            //RESETS
            if StatusModule::status_kind(weapon.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
                NO_PKFIRE[entry_id] = false;
            }
        }
    }
            
pub fn install() {
    smashline::install_agent_frames!(
        //ness_opff, 
        ness_pk_fire_opff);

}