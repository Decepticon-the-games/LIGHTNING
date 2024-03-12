use super::*;

#[skyline::hook(replace = WorkModule::get_param_float)]
pub unsafe fn dyn_param_float_replace(module_accessor: &mut BattleObjectModuleAccessor, param_type: u64, param_hash: u64) ->f32 {
    let ret = original!()(module_accessor, param_type, param_hash);

    //common::general::airdodge::params::airdodge_dyn_float_param(param_type, param_hash).unwrap_or(ret);//Isn't read through get param float...

    common::general::parries::params::parries_dyn_float_param(param_type, param_hash).unwrap_or(ret);
    common::general::dodge::params::remove_dodge_penalty_float_param(param_type, param_hash).unwrap_or(ret);
    common::general::grabs::params::grabs_dyn_float_param(param_type, param_hash).unwrap_or(ret);
    
    //common::params::sdi::remove_sdi_param_float(param_type, param_hash).unwrap_or(ret);
    common::params::aesthetic::aesthetic_float_param(param_type, param_hash).unwrap_or(ret);
    common::params::misc::common_misc_float_param(param_type, param_hash).unwrap_or(ret);
    common::general::shields::params::shields_dyn_float_param(param_type, param_hash).unwrap_or(ret)
}


#[skyline::hook(replace = WorkModule::get_param_int)]
pub unsafe fn handle_get_param_int(module_accessor: &mut BattleObjectModuleAccessor, param_type: u64, param_hash: u64) -> i32 {
    let ret = original!()(module_accessor, param_type, param_hash);

    
    common::general::parries::params::parries_dyn_int_param(param_type, param_hash).unwrap_or(ret);
    common::general::dodge::params::remove_dodge_penalty_int_param(param_type, param_hash).unwrap_or(ret);

    common::params::sdi::remove_sdi_param_int(param_type, param_hash).unwrap_or(ret);
    common::params::buffer::common_precede_int_param(param_type, param_hash).unwrap_or(ret); 
    common::params::misc::common_misc_int_param(param_type, param_hash).unwrap_or(ret);
    common::general::shields::params::shields_dyn_int_param(param_type, param_hash).unwrap_or(ret)
}


pub fn install() {

    skyline::install_hook!(dyn_param_float_replace);

}