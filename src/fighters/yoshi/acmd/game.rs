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

 mod game_Attack11;
 mod game_Attack12;
 mod game_AttackAirN;
 mod game_AttackAirF;
 mod game_AttackAirB;
 mod game_AttackAirHi;
 mod game_AttackAirLw;
 mod game_AttackDash;
 mod game_AttackS3;
 mod game_AttackS3Hi;
 mod game_AttackS3Lw;
 mod game_AttackHi3;
 mod game_AttackLw3;
 mod game_AttackS4;
 mod game_AttackS4Hi;
 mod game_AttackS4Lw;
 mod game_AttackHi4;
 mod game_AttackLw4;
 //mod game_SpecialN;
 mod game_SpecialN2;
 //mod game_SpecialAirN;
 mod game_SpecialAirN2;
 mod game_SpecialHi;
 mod game_SpecialAirHi;
 mod game_SpecialLw;
 mod game_SpecialAirLw;
 mod game_ThrowF;
 mod game_ThrowB;
 mod game_ThrowHi;
 mod game_ThrowLw;
 
 
  pub fn install() {
    game_Attack11::install();
    game_Attack12::install();
    game_AttackAirN::install();
    game_AttackAirF::install();
    game_AttackAirB::install();
    game_AttackAirHi::install();
    game_AttackAirLw::install();
    game_AttackDash::install();
    game_AttackS3::install();
    game_AttackS3Hi::install();
    game_AttackS3Lw::install();
    game_AttackHi3::install();
    game_AttackLw3::install();
    game_AttackS4::install();
    game_AttackS4Hi::install();
    game_AttackS4Lw::install();
    game_AttackHi4::install();
    game_AttackLw4::install();
    //game_SpecialN::install();
    game_SpecialN2::install();
    //game_SpecialAirN::install();
    game_SpecialAirN2::install();
    game_SpecialHi::install();
    game_SpecialAirHi::install();
    game_SpecialLw::install();
    game_SpecialAirLw::install();
    game_ThrowF::install();
    game_ThrowB::install();
    game_ThrowHi::install();
    game_ThrowLw::install();
 }



