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
use crate::fighters::common::mechanics::attack_cancels::{ENABLE_ATTACK_CANCEL,ENABLE_MULTIHIT_CANCEL,MOVEMENT_CANCEL};






#[fighter_frame( agent = FIGHTER_KIND_SAMUSD )]
pub fn samus_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);      
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        //let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);

//
            static mut CANCEL_COUNT : [bool; 8] = [false; 8];
            static mut CANCEL_COUNTER : [i32; 8] = [0; 8];

            /*if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
            || motion_kind == hash40("attack_air_f")
            || motion_kind == hash40("attack_air_hi")
            {
                //if AttackModule::is_attack_occur(fighter.module_accessor) {
                    //Counter 
                    //if CANCEL_COUNT[entry_id] == false {
                    //    CANCEL_COUNTER[entry_id] +=1;
                    //    CANCEL_COUNT[entry_id] = true; 
                    //}
                //}
                //else {
                //    CANCEL_COUNT[entry_id] = false;
                //}
                //Disable cancel
                //if CANCEL_COUNTER[entry_id] >5 {//How many times you can cancel
                //    CANCEL_COUNTER[entry_id] = 6;//How  many hits before disabling cancel
                //    ENABLE_MULTIHIT_CANCEL[entry_id] = false; 
                //}

                //Reset
                //if MOVEMENT_CANCEL[entry_id] {
                //    if CANCEL_COUNTER[entry_id] == 6 {
                //        CANCEL_COUNTER[entry_id] = 0;
                //        ENABLE_MULTIHIT_CANCEL[entry_id] = true; 
                //    }  
                //    MOVEMENT_CANCEL[entry_id] = false;  
                //}
            }

            //Resets
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
                CANCEL_COUNTER[entry_id] = 0;
            }*/
    }
}

pub fn install() {
    smashline::install_agent_frames!(samus_opff);

}