#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]
#![feature(repr_simd)]


mod mechanics;
mod function_hooks;

pub fn install() {

    //general_statuses::install();
    mechanics::install();
    function_hooks::install();
}