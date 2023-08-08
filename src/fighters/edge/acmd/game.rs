use super::*;
use smash::app::FighterSpecializer_Edge::attack_lw4_ray_check;
use crate::fighters::common::mechanics::cancels::counter_cancels::COUNTER_CANCEL;

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
 mod game_AttackS3Hi; 
 mod game_AttackS3Lw;  
 mod game_AttackHi3;
 mod game_AttackLw3;
 mod game_AttackS4;
 mod game_AttackHi4;
 mod game_AttackLw4;

 mod game_SpecialN1;
 mod game_SpecialN2;
 mod game_SpecialNStart;
 mod game_SpecialAirN1;
 mod game_SpecialAirN2;
 mod game_SpecialAirNStart;

 mod game_SpecialS;
 mod game_SpecialAirS;

 mod game_SpecialHi1;
 mod game_SpecialHi2End;
 mod game_SpecialAirHi2End;

 mod game_SpecialLwHit;
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
     game_AttackS3Hi::install();
     game_AttackS3Lw::install();
     game_AttackHi3::install();
     game_AttackLw3::install();
     game_AttackS4::install();    
     game_AttackHi4::install();
     game_AttackLw4::install();

     game_SpecialN1::install();
     game_SpecialN2::install();
     game_SpecialNStart::install();
     game_SpecialAirN1::install();
     game_SpecialAirN2::install();
     game_SpecialAirNStart::install();

     game_SpecialS::install();
     game_SpecialAirS::install();

     game_SpecialHi1::install();
     game_SpecialHi2End::install();
     game_SpecialAirHi2End::install();

     game_SpecialLwHit::install();
     game_SpecialAirLwHit::install();

     game_ThrowF::install();
     game_ThrowB::install();
     game_ThrowHi::install();
     game_ThrowLw::install();
 
 }



