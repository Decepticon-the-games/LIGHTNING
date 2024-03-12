#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

pub mod fighters;
//pub mod utils;
//pub mod dynamic;

#[skyline::main(name = "lightning")]
pub fn main() {
    fighters::install();
    //utils::install();
    //dynamic::install();

}