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

let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;    


ENABLE_ATTACK_CANCEL[entry_id] = true;
CANCEL_IN_NEUTRAL[entry_id] = true;

//JABS

//TILTS

//SMASHATTACKS

//AERIALS

//SPECIALS

//THROWS

pub fn install() {
    smashline::install_acmd_scripts!(
     game_attack11,
     game_attack12,
     game_attackdash,
     game_attacks3,
     game_attacks3hi,
     game_attacks3lw,
     game_attackhi3,
     game_attacklw3,
     game_attacks4,
     game_attackhi4,
     game_attacklw4,
     game_attackairn,
     game_attackairf,
     game_attackairb,
     game_attackairhi,
     game_attackairlw,
     game_specialn,
     game_specialairn,
     game_specials,
     game_specialairs,
     game_specialhi,
     game_specialairhi,
     game_speciallw,
     game_specialairlw,
     game_throwf,
     game_throwb,
     game_throwhi,
     game_throwlw,
     );
 }

















 