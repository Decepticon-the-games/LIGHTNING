use {
    smash::{
        lua2cpp::*,
        app::{lua_bind::*, *},
        lib::lua_const::*
    }
};

pub unsafe fn enable_cancel_real(fighter: &mut L2CFighterCommon) -> bool {
    let terms = [
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LASSO,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_TREAD_JUMP,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_WALL_JUMP,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL
    ];
    let mut is_enabled = [false; 18];
    for x in 0..terms.len() {
        is_enabled[x] = WorkModule::is_enable_transition_term_group(fighter.module_accessor, terms[x]);
        WorkModule::enable_transition_term_group(fighter.module_accessor, terms[x]);
    }
    let is_crouching = [
        *FIGHTER_STATUS_KIND_ATTACK_LW3
    ].contains(&StatusModule::status_kind(fighter.module_accessor));
    let cancel = fighter.sub_wait_ground_check_common(is_crouching.into()).get_bool()
    || fighter.sub_air_check_fall_common().get_bool();
    for x in 0..terms.len() {
        if !is_enabled[x] {
            WorkModule::unable_transition_term_group(fighter.module_accessor, terms[x]);
        }
    }
    cancel
}