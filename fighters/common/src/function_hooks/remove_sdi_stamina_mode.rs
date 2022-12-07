use super::*;

//REMOVE SDI IN STAMINA MODE

#[skyline::hook(offset = 0x4E53C0)]
pub unsafe fn get_param_int_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let _fighter_kind = utility::get_kind(boma);
    if param_hash == hash40("hit_stop_delay_flick") {
        if FighterUtil::is_hp_mode(boma) {
            return 0x4;
        }
    }
    //return ret;
    //if param_hash == hash40("precede") { //No buffer during neutral at all until comboing (attacks hit)
    //    if AttackModule::is_attack_occur(boma) {
    //        return 0x10;
    //    }
    //    else{
    //        return 0x1;
    //    }
    //}
    return ret;
}

#[skyline::hook(offset = 0x4E5380)]
pub unsafe fn get_param_float_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let _fighter_kind = utility::get_kind(boma);
    if param_hash == hash40("hit_stop_delay_stick") {
        if FighterUtil::is_hp_mode(boma) {
            return 0.7;
        }
    }

    return ret;
    
}
pub fn install() {
    //skyline::install_hook!(get_param_float_replace);
    skyline::install_hook!(get_param_int_replace);
}