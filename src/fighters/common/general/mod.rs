pub mod parries;
pub mod shields;
pub mod airdodge;
pub mod dodge;
//pub mod traction;
//pub mod fs_meter;

pub fn install() {
    parries::install(); 
    shields::install();       
    airdodge::install();
    dodge::install();
    //traction::install();
    //fs_meter::install();
}