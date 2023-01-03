use {
    smash::{
        lua2cpp::*,
        app::{lua_bind::*, *},
        lib::lua_const::*
    }
};

pub unsafe fn enable_cancel_real(fighter: &mut L2CFighterCommon) -> bool {
    let groups = [
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP,
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
    let ids = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH,
    ];
    let mut is_group_enabled = [false; 18];
    let mut is_id_enabled = [false; 4];
    for x in 0..groups.len() {
        is_group_enabled[x] = WorkModule::is_enable_transition_term_group(fighter.module_accessor, groups[x]);
        WorkModule::enable_transition_term_group(fighter.module_accessor, groups[x]);
    }
    for x in 0..ids.len() {
        is_id_enabled[x] = WorkModule::is_enable_transition_term_group(fighter.module_accessor, ids[x]);
        WorkModule::enable_transition_term(fighter.module_accessor, ids[x]);
    }
    let is_crouching = [
        *FIGHTER_STATUS_KIND_ATTACK_LW3
    ].contains(&StatusModule::status_kind(fighter.module_accessor));
    let cancel = fighter.sub_wait_ground_check_common(is_crouching.into()).get_bool()
    || fighter.sub_air_check_fall_common().get_bool();
    for x in 0..groups.len() {
        if !is_group_enabled[x] {
            WorkModule::unable_transition_term_group(fighter.module_accessor, groups[x]);
        }
    }
    for x in 0..ids.len() {
        if !is_id_enabled[x] {
            WorkModule::unable_transition_term(fighter.module_accessor, ids[x]);
        }
    }
    cancel
}