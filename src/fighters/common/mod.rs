#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]
#![feature(repr_simd)]


pub mod mechanics;
mod function_hooks;
//pub mod utility;

pub fn install() {

    //general_statuses::install();
    mechanics::install();
    function_hooks::install();
}