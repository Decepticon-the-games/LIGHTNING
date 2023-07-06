#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
//#![allow(unused_macros)]

pub mod fighters;
//pub mod utils;
//pub mod dynamic;

#[skyline::main(name = "lightning")]
pub fn main() {
    fighters::install();
    //utils::install();
    //dynamic::install();
}