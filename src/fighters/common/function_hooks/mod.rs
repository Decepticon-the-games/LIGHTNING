use super::*;

pub mod notify_collision_event;
pub mod float_int_hook;
pub mod transition_terms;
//pub mod hitbox_effect_replace;
//pub mod get_param_replace;

pub fn install() {
    notify_collision_event::install();
    float_int_hook::install();
    transition_terms::install();   
    //hitbox_effect_replace::install();   
    //get_param_replace::install();   
}
