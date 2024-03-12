use super::*;

pub mod upbtransitions;
pub mod edgecancelling;
pub mod moonwalking;


pub fn install() {
    upbtransitions::install();
    edgecancelling::install();
    moonwalking::install();
}

