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
 mod game_AttackS4Hi;
 mod game_AttackS4Lw; 
 mod game_AttackHi4;
 mod game_AttackLw4;

 mod game_SpecialS;
 mod game_SpecialSLanding;
 mod game_SpecialAirS;
 mod game_SpecialAirSFall;

 mod game_SpecialHiAdd;

 mod game_SpecialLw;
 mod game_SpecialLwAttack;
 mod game_SpecialLwHit;
 mod game_SpecialAirLw;
 mod game_SpecialAirLwAttack;
 mod game_SpecialAirLwHit;

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
     game_AttackS4Hi::install();
     game_AttackS4Lw::install(); 
     game_AttackHi4::install();
     game_AttackLw4::install();
    
     game_SpecialS::install();
     game_SpecialSLanding::install();
     game_SpecialAirS::install();
     game_SpecialAirSFall::install();
    
     game_SpecialHiAdd::install();
    
     game_SpecialLw::install();
     game_SpecialLwAttack::install();
     game_SpecialLwHit::install();
     game_SpecialAirLw::install();
     game_SpecialAirLwAttack::install();
     game_SpecialAirLwHit::install();
    
     game_ThrowF::install();
     game_ThrowB::install();
     game_ThrowHi::install();
     game_ThrowLw::install();
     
 
 }



