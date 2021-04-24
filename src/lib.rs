#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod lightning_ultrainstinct;
mod lightning_edge;
mod lightning_pit;
mod lightning_cloud;
mod lightning_falco_fox;
mod lightning_ganon;
mod lightning_captain;
mod lightning_koopa;
mod lightning_link;
mod lightning_samus_samusd;
mod lightning_marth_lucina;
mod lightning_roy_chrom;
mod lightning_szerosuit;
mod lightning_littlemac;
mod lightning_shulk;
mod lightning_jack;
mod lightning_simon_richter;
mod detector_boxes;
mod lightning_common;
mod lightning_counter_cancels;
mod lightning_motioncancels;
mod lightning_upbcancels;
mod lightning_special_tech;
//mod lightning_upbtransitions;

#[skyline::main(name = "acmd_test")]
pub fn main() {
    lightning_ultrainstinct::install();   
    detector_boxes::install();
    lightning_edge::install();
    lightning_pit::install();
    lightning_cloud::install();
    lightning_falco_fox::install();
    lightning_captain::install();
    lightning_ganon::install();
    lightning_koopa::install();
    lightning_link::install();
    lightning_samus_samusd::install();
    lightning_marth_lucina::install();
    lightning_roy_chrom::install();
    lightning_szerosuit::install();
    lightning_littlemac::install();
    lightning_shulk::install();
    lightning_jack::install();
    lightning_simon_richter::install();
    lightning_common::install();
    lightning_counter_cancels::install();
    lightning_motioncancels::install();
    lightning_upbcancels::install();
    lightning_special_tech::install();
    //lightning_upbtransitions::install();
}