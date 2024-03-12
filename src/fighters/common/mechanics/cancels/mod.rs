use super::*;

pub mod attack_cancels;
pub mod counter_cancels;
pub mod motioncancels;

pub fn install() {

    attack_cancels::install();
    counter_cancels::install();
    motioncancels::install();
}