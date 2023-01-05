
use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        phx::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*
};



pub static mut DEGREES : [f32; 8] = [0.0; 8];

#[skyline::hook(replace = L2CFighterCommon_setup_escape_air_slide_common)]

//AIRDODGE EFFECTS
pub unsafe fn escape_air_slide_effecs(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue) {

    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let stick_degrees = ControlModule::get_stick_angle(fighter.module_accessor).to_degrees();

    DEGREES[entry_id] = stick_degrees;

}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            escape_air_slide_effecs
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}