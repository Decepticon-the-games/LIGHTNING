use super::*;

 mod game_Attack11;
 mod game_Attack12;
 mod game_Attack13;
 mod game_Attack100End; 
 mod game_AttackAirN;
 mod game_AttackAirF;
 mod game_AttackAirHi;
 mod game_AttackAirLw;
 mod game_AttackDash;
 mod game_AttackHi3;
 mod game_AttackLw3;
 mod game_AttackHi4;
 mod game_AttackLw4;

 mod game_SpecialNEnd;
 mod game_SpecialAirNEnd;

 mod game_SpecialAirHiEnd;
 mod game_SpecialAirHiEnd2;
 mod game_SpecialHiLongEnd;
 mod game_SpecialHiShortEnd;

 mod game_ThrowF;
 mod game_ThrowB;
 mod game_ThrowHi;
 mod game_ThrowLw;
 
 
  pub fn install() {
     game_Attack11::install();
     game_Attack12::install();
     game_Attack13::install();
     game_Attack100End::install(); 
     game_AttackAirN::install();
     game_AttackAirF::install();
     game_AttackAirHi::install();
     game_AttackAirLw::install();
     game_AttackDash::install();
     game_AttackHi3::install();
     game_AttackLw3::install(); 
     game_AttackHi4::install();
     game_AttackLw4::install();

     //game_SpecialNEnd::install();
     //game_SpecialAirNEnd::install();
    
     game_SpecialAirHiEnd::install();
     game_SpecialAirHiEnd2::install();
     game_SpecialHiLongEnd::install();
     game_SpecialHiShortEnd::install();

     game_ThrowF::install();
     game_ThrowB::install();
     game_ThrowHi::install();
     game_ThrowLw::install();
 
 }



