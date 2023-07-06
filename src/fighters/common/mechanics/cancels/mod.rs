
pub mod attack_cancels;
pub mod counter_cancels;
pub mod detector_boxes;
pub mod motioncancels;
pub mod upbcancels;

pub fn install() {

    attack_cancels::install();
    counter_cancels::install();
    detector_boxes::install();
    motioncancels::install();
    upbcancels::install();
}