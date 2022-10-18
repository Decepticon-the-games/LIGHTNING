use crate::lightning_01_custom_meters::crimson_meter::crimson_meter_smashline_agent_frame_callback_install;
use crate::lightning_01_custom_meters::cross_meter::cross_meter_smashline_agent_frame_callback_install;
use crate::lightning_01_custom_meters::vanish_meter::vanish_meter_smashline_agent_frame_callback_install;

mod crimson_meter;
mod cross_meter;
mod vanish_meter;

pub fn install() {
    smashline::install_agent_frame_callbacks!(crimson_meter);
    smashline::install_agent_frame_callbacks!(cross_meter);
    smashline::install_agent_frame_callbacks!(vanish_meter);
}