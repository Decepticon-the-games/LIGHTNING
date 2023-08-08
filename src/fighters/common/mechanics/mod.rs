use super::*;

pub mod cancels;
pub mod lightning_mechanics;
pub mod misc;




pub fn install() {
    cancels::install(); 
    lightning_mechanics::install();       
    misc::install();
}