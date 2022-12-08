use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::hash40; 
use skyline::hooks::{getRegionAddress, Region};
use smash::app::FighterManager;

pub mod cross_cancel_vanish;
mod remove_sdi_stamina_mode;
mod transition_terms;

pub fn install() {
    cross_cancel_vanish::install();
    remove_sdi_stamina_mode::install();
    transition_terms::install();    
}
