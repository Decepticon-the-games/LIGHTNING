#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
//#![allow(unused_macros)]

pub mod fighters;


#[skyline::main(name = "lightning")]
pub fn main() {
    fighters::install();
}