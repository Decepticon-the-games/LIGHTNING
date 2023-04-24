#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]
// use ::common::prelude::*;

pub mod acmd;

//pub mod status;
//pub mod opff;


pub fn install() {
    acmd::install();
    //status::install();
    //opff::install();
}