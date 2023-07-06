
pub mod cross_cancel_vanish;
pub mod float_int_hook;
pub mod transition_terms;
//pub mod hitbox_effect_replace;

pub fn install() {
    cross_cancel_vanish::install();
    float_int_hook::install();
    transition_terms::install();   
    //hitbox_effect_replace::install();   
}
