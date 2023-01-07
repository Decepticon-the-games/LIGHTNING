use super::*;

//mod common;
pub mod attack_cancels;
pub mod counter_cancels;
pub mod detector_boxes;
pub mod motioncancels;
pub mod crimson_cancel;
pub mod lightning_mode;
pub mod ultrainstinct;
pub mod upbcancels;
pub mod upbtransitions;
pub mod edgecancelling;
//mod up_special_callbacks;
pub mod lightning_fsmeter;
//mod custom_meters;
pub mod vanish;
pub mod misc;

pub fn install() {
    attack_cancels::install();
    counter_cancels::install();
    detector_boxes::install();
    motioncancels::install();
    crimson_cancel::install();
    lightning_mode::install();
    ultrainstinct::install();
    upbcancels::install();
    upbtransitions::install();
    edgecancelling::install();
    //vanish::install();
    misc::install();
}