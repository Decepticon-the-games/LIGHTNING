use super::*;
use smash::app::sv_module_access::item;

 mod game_Attack11;
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

 mod game_SpecialNFire;
 //mod game_SpecialNUse2;
 mod game_SpecialAirNFire;
 //mod game_SpecialAirNUse2;

 mod game_SpecialSThrowB;
 mod game_SpecialSThrowF;
 mod game_SpecialSThrowHi;
 mod game_SpecialSThrowLw;
 mod game_SpecialAirSThrowB;
 mod game_SpecialAirSThrowF;
 mod game_SpecialAirSThrowHi;
 mod game_SpecialAirSThrowLw; 

 mod game_SpecialAirHiEnd;

 mod game_SpecialLwSet;

 mod game_ThrowF;
 mod game_ThrowB;
 mod game_ThrowHi;
 mod game_ThrowLw;
 
 
  pub fn install() {
     game_Attack11::install();
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
    
     game_SpecialNFire::install();
     //game_SpecialNUse2::install();
     game_SpecialAirNFire::install();
     //game_SpecialAirNUse2::install();
    
     game_SpecialSThrowB::install();
     game_SpecialSThrowF::install();
     game_SpecialSThrowHi::install();
     game_SpecialSThrowLw::install();
     game_SpecialAirSThrowB::install();
     game_SpecialAirSThrowF::install();
     game_SpecialAirSThrowHi::install();
     game_SpecialAirSThrowLw::install(); 
    
     game_SpecialAirHiEnd::install();
    
     game_SpecialLwSet::install();
    
     game_ThrowF::install();
     game_ThrowB::install();
     game_ThrowHi::install();
     game_ThrowLw::install();
 
 }



