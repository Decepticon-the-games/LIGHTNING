use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        hash40,
        app::{lua_bind::*, sv_animcmd::*,*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

use crate::fighters::common::function_hooks::float_int_hook::{PARAM_INT_OFFSET, PARAM_FLOAT_OFFSET};

#[skyline::hook(offset = PARAM_INT_OFFSET)]
pub unsafe fn common_precede_int_param_accessor_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);

    if param_hash == hash40("precede") { //1 buffer during neutral at all until comboing (buffer 10 on attacks hit)
        

        if AttackModule::is_attack_occur(boma) {
            
            return 10;
        }
        else{
            return 1;
        }
    }
    if param_hash == hash40("precede_extension") {//NO HOLD BUFFER
        return 0;
    }
    ret
}
pub fn install() {
    skyline::install_hooks!(
        common_precede_int_param_accessor_hook,
    );
}