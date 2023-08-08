use super::*;

pub static mut ILLUSION_CANCEL : [bool; 8] = [false; 8];
pub static mut FASTFALL_LASER : [bool; 8] = [false; 8];


pub mod acmd;

//pub mod status;
//pub mod opff;
//pub mod params;


pub fn install() {
    acmd::install();
    //status::install();
    //opff::install();
    //params::install();
}