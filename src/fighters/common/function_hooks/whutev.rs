#[skyline::hook(offset=INT_OFFSET)]
pub unsafe fn int_param_accessor_hook_toon(
boma: u64,
param_type: u64,
param_hash: u64) -> i32{
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(boma, param_type, param_hash);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);

    // Toon Link Variable Timer
    let flick =  ControlModule::get_flick_y(module_accessor);
    let stick = ControlModule::get_stick_y(module_accessor);
    if fighter_kind == FIGHTER_KIND_TOONLINK
    && flick <= 5 
    && stick <= -0.8
    && param_hash ==  hash40("bomb") {
        println!("Flick");
        if param_type == hash40("toonlinkbomb_limit_frame") {
            println!("Short Timer");
            return 60; //original 300
        }
    }
    
ret
}
