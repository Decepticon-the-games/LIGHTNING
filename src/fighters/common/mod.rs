
pub mod mechanics;
pub mod function_hooks;
pub mod general;
//pub mod params;

pub fn install() {

    general::install();
    //params::install();
    mechanics::install();
    function_hooks::install();
}