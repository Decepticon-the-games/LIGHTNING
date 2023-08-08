use super::*;

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

 mod game_SpecialNThrowHi;
 mod game_SpecialNThrowLw;
 mod game_SpecialNThrowM;
 mod game_SpecialAirNThrowHi;
 mod game_SpecialAirNThrowLw;
 mod game_SpecialAirNThrowM;

 //mod game_SpecialS;
 //mod game_SpecialAirS;

 //mod game_SpecialHi;
 //mod game_SpecialAirHi;

 mod game_SpecialLwWall;
 mod game_SpecialLwEnemy; 
 mod game_SpecialLwGround; 
 mod game_SpecialAirLwEnemy;
 mod game_SpecialAirLwGround; 
 mod game_SpecialAirLwWall; 

 

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
   
    game_SpecialNThrowHi::install();
    game_SpecialNThrowLw::install();
    game_SpecialNThrowM::install();
    game_SpecialAirNThrowHi::install();
    game_SpecialAirNThrowLw::install();
    game_SpecialAirNThrowM::install();
   
    //game_SpecialS::install();
    //game_SpecialAirS::install();
   
    //game_SpecialHi::install();
    //game_SpecialAirHi::install();
   
    game_SpecialLwWall::install();
    game_SpecialLwEnemy::install(); 
    game_SpecialLwGround::install(); 
    game_SpecialAirLwEnemy::install();
    game_SpecialAirLwGround::install(); 
    game_SpecialAirLwWall::install(); 
   
    
   
    game_ThrowF::install();
    game_ThrowB::install();
    game_ThrowHi::install();
    game_ThrowLw::install();
 
 }



