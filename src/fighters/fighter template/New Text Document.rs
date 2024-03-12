use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::{Hash40,Vector3f},
        app::{lua_bind::*, sv_animcmd::*,*},
        lib::lua_const::*,
    },
    smash_script::*,
    smashline::*
};
use crate::fighters::{
    common::{
        mechanics::{
            attack_cancels::{
                ENABLE_ATTACK_CANCEL,ENABLE_MULTIHIT_CANCEL,MOVEMENT_CANCEL
            },
            motioncancels::{
                CANCEL_IN_NEUTRAL
            }
        }
    }
};

use super::*;
use crate::upbtransitions::DISABLE_UP_SPECIAL;
use crate::fighters::common::mechanics::cancels::counter_cancels::COUNTER_CANCEL;

let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

if AttackModule::is_attack_occur(fighter.module_accessor) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
}
if AttackModule::is_attack_occur(fighter.module_accessor) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
}


    //enable_attack_cancel(fighter); 
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_CATEGORY_MASK_ALL) {
            enable_attack_cancel(fighter); 
        }


    ENABLE_MULTIHIT_CANCEL[entry_id] = true;

    whiff_cancel(fighter);


    COUNTER_CANCEL[entry_id] = true; 

    ENABLE_MULTIHIT_COUNT[entry_id];

pub fn install() {
    smashline::install_acmd_scripts!(
    game_);
}
 
//multihit cancels

        //Cancel up special only after 3 hits


        //In Lightning...
        if IS_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING[entry_id] {
            //Up B cancels after 3 successful hits, cancel into jabs, tilts, smashes, neutral/side b     
            let next_input = //cat1 flag input
            multihit_cancel(fighter, /*i32*/, /*hash40*/, /*hitcount*/, next_input, /*reset_i32*/, /*reset_hash40*/);
        }
