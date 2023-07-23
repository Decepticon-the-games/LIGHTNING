use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::{Hash40,Vector3f,Vector2f},
        app::{lua_bind::*, sv_animcmd::*,*},
        lib::lua_const::*,
        hash40
    },
    smash_script::*,
    smashline::*
};
use crate::fighters::{
    common::{
        mechanics::{
            cancels::{
                attack_cancels::{
                    ENABLE_ATTACK_CANCEL,ENABLE_MULTIHIT_CANCEL,MOVEMENT_CANCEL
                },
                motioncancels::{
                    CANCEL_IN_NEUTRAL
                }                
            }
        }
    }
};

 mod game_Attack11;
 mod game_Attack12;
 mod game_Attack13;
 mod game_AttackAirN;
 mod game_AttackAirF;
 mod game_AttackAirB;
 mod game_AttackAirHi;
 mod game_AttackAirLw;
 mod game_AttackDash;
 mod game_AttackS3;
 mod game_AttackHi3;
 mod game_AttackLw3;
 mod game_AttackS4;
 mod game_AttackHi4;
 mod game_AttackLw4;

 mod game_SpecialN1;
 mod game_SpecialN2;
 mod game_SpecialN3End;
 mod game_SpecialN3EndMax;
 mod game_SpecialN3EndMaxTurn;
 mod game_SpecialN3EndTurn;
 mod game_SpecialAirN1;
 mod game_SpecialAirN2;
 mod game_SpecialAirN3End;
 mod game_SpecialAirN3EndMax;
 mod game_SpecialAirN3EndMaxTurn;
 mod game_SpecialAirN3EndTurn;

 mod game_SpecialS1;
 mod game_SpecialS1End;
 mod game_SpecialS1Hit;
 mod game_SpecialS2Attack;
 mod game_SpecialS3_1;
 mod game_SpecialS3_1Hi;
 mod game_SpecialS3_1Lw;
 mod game_SpecialAirS1;
 mod game_SpecialAirS1End;
 mod game_SpecialAirS2Attack;
 mod game_SpecialAirS3_1;
 mod game_SpecialAirS3_1Hi;
 mod game_SpecialAirS3_1Lw;

 mod game_SpecialHi1;
 mod game_SpecialHi1End;
 mod game_SpecialHi2;
 mod game_SpecialHi3;
 mod game_SpecialAirHi1;
 mod game_SpecialAirHi1End;
 mod game_SpecialAirHi3;

 mod game_SpecialLw1;
 mod game_SpecialLw1Hit;
 mod game_SpecialLw2;
 mod game_SpecialLw3;
 mod game_SpecialLw3End;
 mod game_SpecialAirLw1;
 mod game_SpecialAirLw1Hit;
 mod game_SpecialAirLw2;
 mod game_SpecialAirLw3;
 mod game_SpecialAirLw3End;

 mod game_ThrowF;
 mod game_ThrowB;
 mod game_ThrowHi;
 mod game_ThrowLw;
 
 
  pub fn install() {

     game_Attack11::install();
     game_Attack12::install();
     game_Attack13::install();
     game_AttackAirN::install();
     game_AttackAirF::install();
     game_AttackAirB::install();
     game_AttackAirHi::install();
     game_AttackAirLw::install();
     game_AttackDash::install();
     game_AttackS3::install();
     game_AttackHi3::install();
     game_AttackLw3::install();
     game_AttackS4::install();
     game_AttackHi4::install();
     game_AttackLw4::install();
    
     game_SpecialN1::install();
     game_SpecialN2::install();
     game_SpecialN3End::install();
     game_SpecialN3EndMax::install();
     game_SpecialN3EndMaxTurn::install();
     game_SpecialN3EndTurn::install();
     game_SpecialAirN1::install();
     game_SpecialAirN2::install();
     game_SpecialAirN3End::install();
     game_SpecialAirN3EndMax::install();
     game_SpecialAirN3EndMaxTurn::install();
     game_SpecialAirN3EndTurn::install();
    
     //game_SpecialS1::install();
     game_SpecialS1End::install();
     game_SpecialS1Hit::install();
     game_SpecialS2Attack::install();
     game_SpecialS3_1::install();
     game_SpecialS3_1Hi::install();
     game_SpecialS3_1Lw::install();
     game_SpecialAirS1::install();
     game_SpecialAirS1End::install();
     game_SpecialAirS2Attack::install();
     game_SpecialAirS3_1::install();
     game_SpecialAirS3_1Hi::install();
     game_SpecialAirS3_1Lw::install();
    
     game_SpecialHi1::install();
     game_SpecialHi1End::install();
     game_SpecialHi2::install();
     game_SpecialHi3::install();
     game_SpecialAirHi1::install();
     game_SpecialAirHi1End::install();
     game_SpecialAirHi3::install();
    
     game_SpecialLw1::install();
     game_SpecialLw1Hit::install();
     game_SpecialLw2::install();
     game_SpecialLw3::install();
     game_SpecialLw3End::install();
     game_SpecialAirLw1::install();
     game_SpecialAirLw1Hit::install();
     game_SpecialAirLw2::install();
     game_SpecialAirLw3::install();
     game_SpecialAirLw3End::install();
    
     game_ThrowF::install();
     game_ThrowB::install();
     game_ThrowHi::install();
     game_ThrowLw::install();
 
 }



