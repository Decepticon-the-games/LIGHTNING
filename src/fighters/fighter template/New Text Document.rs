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

let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;


   ENABLE_ATTACK_CANCEL[entry_id] = true; 
} 
if ! (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
   ENABLE_ATTACK_CANCEL[entry_id] = false;  
}

if ENABLE_MULTIHIT_CANCEL[entry_id] {
    ENABLE_ATTACK_CANCEL[entry_id] = true;
}

    CANCEL_IN_NEUTRAL[entry_id] = true;
}

}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_ );
}

//multihit cancels

if ENABLE_MULTIHIT_CANCEL[entry_id] && LIGHTNING {
    
}

static mut MULTIHIT : [bool; 8] = [false; 8];
static mut MULTIHIT_COUNT : [i32; 8] = [0; 8];



if motion_kind == hash40(/*motion here*/)  {
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
        if MULTIHIT[entry_id] == false {
            MULTIHIT_COUNT[entry_id] +=1;
            MULTIHIT[entry_id] = true; 
        }         
    }
    else {
        MULTIHIT[entry_id] = false;
    }  

    if MULTIHIT_COUNT[entry_id] >= 3 { //how many hits
        MULTIHIT_COUNT[entry_id] = 3;  //how many hits
        ENABLE_MULTIHIT_CANCEL[entry_id] = true; 
    }
    else {
        ENABLE_MULTIHIT_CANCEL[entry_id] = false;
    } 
}
else {
    MULTIHIT_COUNT[entry_id] = 0;
}