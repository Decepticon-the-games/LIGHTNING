use super::*;

pub mod parries;
pub mod shields;
pub mod airdodge;
pub mod dodge;
pub mod dash;
pub mod jumpsquat;
pub mod grabs;
//pub mod fs_meter;

pub fn install() {
    parries::install(); 
    shields::install();       
    airdodge::install();
    dodge::install();
    dash::install();
    jumpsquat::install();

    //fs_meter::install();
}