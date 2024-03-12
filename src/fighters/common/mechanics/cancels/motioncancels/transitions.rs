use super::*;

pub unsafe fn moveset_transition_terms(fighter : &mut L2CFighterCommon) -> bool {

    //Jabs
    term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100
    term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH
    //Tilts
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3
    //Aerials
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR
    //Smash Attacks
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_HOLD
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_HOLD
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_HOLD
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START
    //Specials
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
    //Command Attacks
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND
    || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB
    
}

pub unsafe fn is_out_of_dash_transitions(fighter : &mut L2CFighterCommon, ) -> bool {

    || WorkModule::is_enable_transition_term(fighter.module_accessor, || *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH)
    || WorkModule::is_enable_transition_term(fighter.module_accessor, || *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT)
    || WorkModule::is_enable_transition_term(fighter.module_accessor, || *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON)
    || WorkModule::is_enable_transition_term(fighter.module_accessor, || *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH)

}