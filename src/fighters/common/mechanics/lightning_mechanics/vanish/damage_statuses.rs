use smash::app::BattleObjectModuleAccessor;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;

pub unsafe fn is_damage_status(module_accessor: &mut smash::app::BattleObjectModuleAccessor, is_prev: bool) -> bool{

    if [*FIGHTER_STATUS_KIND_DAMAGE, 
        *FIGHTER_STATUS_KIND_DAMAGE_AIR, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY, 
        *FIGHTER_STATUS_KIND_DAMAGE_FALL, 
        *FIGHTER_STATUS_KIND_DAMAGE_SONG, 
        *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
        ].contains(&status_kind) {
        true
    }
    else{
        false
    }
}