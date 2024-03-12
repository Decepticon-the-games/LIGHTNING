use super::*;

/*
Whiff Cancel into dash, jump, dodges, and shield.

Set a variable CANCEL_IN_NEUTRAL. 
This gets turned off once you're in the desired status, when the current attack's animation is done, or just entering any other status.
Dash disables attack/special transition terms, so dash animation plays out. Returns to normal after dash is done.
Jump/Airdodge disables jump/airdodge transitions respectively once you reach your max /jump/airdodge count.
Dodges...
Shield disables the ability to shield for a longer duration.
*/

pub unsafe fn whiff_cancel(fighter : &mut L2CAgentBase) {//This function goes out to ACMD moveset  scrips so they can read this code.
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);

    if entry_id < 8 {

        if ! (AttackModule::is_attack_occur(fighter.module_accessor) && status_kind != *FIGHTER_STATUS_KIND_THROW) {
            CANCEL_IN_NEUTRAL[entry_id] = true;
        }
        else if ATTACK_CANCEL[entry_id] {
            CANCEL_IN_NEUTRAL[entry_id] = false;
        }        
    }
}
#[fighter_frame_callback]
pub fn landing_lag(fighter : &mut L2CFighterCommon) {  
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);

        if (status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR || status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL) && frame >1.0 {
            if IS_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING[entry_id] {
                //CancelModule::enable_cancel(fighter.module_accessor);
            }
            else {
                CANCEL_IN_NEUTRAL[entry_id] = true; 
            }
        }
    }
}

#[fighter_frame_callback]
pub fn cancel_in_neutral(fighter : &mut L2CFighterCommon) {   
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(fighter.module_accessor);

        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);

        let frame = MotionModule::frame(fighter.module_accessor);

        let max_jumps = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        let edgde_one_wing_max_jumps = WorkModule::get_int(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_JUMP_COUNT_MAX);
        let jumps_used = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        
        let grab = (motion_kind == smash::hash40("catch"))
        || (motion_kind == smash::hash40("catch_dash"))
        || (motion_kind == smash::hash40("catch_turn"));

        //Enable whiff cancel on a landindg aerial/special fall landing
        if (status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR || status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL) && frame >1.0 {
            CANCEL_IN_NEUTRAL [entry_id] = true; 
            //CancelModule::enable_cancel(fighter.module_accessor);
        }  
    
        if CANCEL_IN_NEUTRAL[entry_id] 
        {
            if situation_kind == *SITUATION_KIND_AIR {
                AIRDODGE_PLUS[entry_id] = true;//Enable multiple airdadges, therefore cancelling into multiple airdodges at a time.  
            }
            
            if [
                *FIGHTER_STATUS_KIND_DASH,
                *FIGHTER_STATUS_KIND_TURN_DASH,
                *FIGHTER_STATUS_KIND_JUMP,
                *FIGHTER_STATUS_KIND_JUMP_SQUAT,
                *FIGHTER_STATUS_KIND_JUMP_AERIAL,         
                *FIGHTER_STATUS_KIND_GUARD,
                *FIGHTER_STATUS_KIND_GUARD_ON,
                *FIGHTER_STATUS_KIND_ESCAPE, 
                *FIGHTER_STATUS_KIND_ESCAPE_F, 
                *FIGHTER_STATUS_KIND_ESCAPE_B            
            ].contains(&status_kind) 
            || [smash::hash40("escape_air"), smash::hash40("escape_air_slide")].contains(&motion_kind)
            {
                ControlModule::set_command_life_extend(fighter.module_accessor, 7 as u8);
                //AIRDODGE
                if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind) {
                    //For as many AIRDODGES as you have...
                    if (max_jumps == 2 && AIRDODGE_COUNT[entry_id] <2)
                    || (max_jumps == 3 && AIRDODGE_COUNT[entry_id] <3) 
                    || (max_jumps == 4 && AIRDODGE_COUNT[entry_id] <4) 
                    || (max_jumps == 5 && AIRDODGE_COUNT[entry_id] <5) 
                    || (max_jumps == 6 && AIRDODGE_COUNT[entry_id] <6)
                    || (edgde_one_wing_max_jumps == 3 && AIRDODGE_COUNT[entry_id] <3)
                    {
                        whiff_cancel_effects(fighter); 
                    }
                }
                //JUMPS
                if [*FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&status_kind) {
                    whiff_cancel_effects(fighter);
                    if [*FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&status_kind) {
                        WHIFF_CANCEL_EFFECTS[entry_id] = true;
                    }
                }
                //DASH
                if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) {
                    whiff_cancel_effects(fighter);
                    WHIFF_CANCEL_INTO_DASH[entry_id] = true; 
                }
                //SHIELD
                if [*FIGHTER_STATUS_KIND_GUARD_ON].contains(&status_kind) {
                    whiff_cancel_effects(fighter);
                    WHIFF_CANCEL_INTO_SHIELD[entry_id] = true; 
                    WHIFF_CANCEL_EFFECTS[entry_id] = true;
                }
                //DODGES
                if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&status_kind) {
                    whiff_cancel_effects(fighter); 
                }
                CANCEL_IN_NEUTRAL [entry_id] = false;
            }   
            else if StopModule::is_hit(fighter.module_accessor) {
                CANCEL_IN_NEUTRAL [entry_id] = false;
            }             
            else if MotionModule::frame(fighter.module_accessor) == FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,smash::phx::Hash40::new_raw(motion_kind),false) {
                CANCEL_IN_NEUTRAL [entry_id] = false;
            } 
            else {
                CancelModule::enable_cancel(fighter.module_accessor); //When in cancel in neutral, other transition terms are disabled so u can only cancel into dash, jump, dodges, and shield. See transition_terms.rs
            }
        }
        if [*FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_GUARD].contains(&status_kind) && WHIFF_CANCEL_EFFECTS[entry_id] {
            whiff_cancel_effects(fighter); 
        }
    } 
}


//EFFECTS
pub unsafe fn whiff_cancel_effects(fighter : &mut L2CFighterCommon) {

    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let zero = smash::phx::Vector3f {x:0.0,y:0.0,z:0.0};
    if WHIFF_CANCEL_EFFECTS[entry_id] {
        //EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_sp_flash"), smash::phx::Hash40::new("waist"), &zero, &zero, 0.7, &zero, &zero, false, 0, 0, 0);
        //macros::LAST_EFFECT_SET_COLOR(fighter,0.0, 0.784, 1.0,);
        macros::BURN_COLOR(fighter, 0.0, 0.784, 1.0, 0.5);    
        WHIFF_CANCEL_EFFECTS[entry_id] = false;
    }
    
    if [
    *FIGHTER_STATUS_KIND_JUMP_SQUAT,  
    *FIGHTER_STATUS_KIND_GUARD_ON,  
    ].contains(&status_kind) {
        //EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_sp_flash"), smash::phx::Hash40::new("waist"), &zero, &zero, 0.7, &zero, &zero, false, 0, 0, 0);
        //macros::LAST_EFFECT_SET_COLOR(fighter,0.0, 0.784, 1.0,);
        macros::BURN_COLOR(fighter, 0.0, 0.784, 1.0, 0.5);    
    }
    if [
    *FIGHTER_STATUS_KIND_JUMP,
    *FIGHTER_STATUS_KIND_JUMP_AERIAL
    ].contains(&status_kind) {
        //EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_sp_flash"), smash::phx::Hash40::new("waist"), &zero, &zero, 0.7, &zero, &zero, false, 0, 0, 0);
        //macros::LAST_EFFECT_SET_COLOR(fighter,0.0, 0.784, 1.0,);
        macros::BURN_COLOR(fighter, 0.0, 0.784, 1.0, 0.5);    
    }
    if [
    *FIGHTER_STATUS_KIND_GUARD,
    *FIGHTER_STATUS_KIND_ESCAPE, 
    *FIGHTER_STATUS_KIND_ESCAPE_F, 
    *FIGHTER_STATUS_KIND_ESCAPE_B
    ].contains(&status_kind) {
        macros::BURN_COLOR(fighter, 0.0, 0.784, 1.0, 0.5);
    }
    if [
    *FIGHTER_STATUS_KIND_DASH,
    *FIGHTER_STATUS_KIND_TURN_DASH,         
    ].contains(&status_kind) {
        if ControlModule::get_stick_angle(fighter.module_accessor).to_degrees() >= 91.0 && ControlModule::get_stick_angle(fighter.module_accessor).to_degrees() <= 270.0 {
            let rotation = smash::phx::Vector3f {x:0.0,y:0.0,z: 90.0};
            //EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_jump_aerial"), smash::phx::Hash40::new("waist"), &zero, &rotation, 1.0, &zero, &zero, false, 0, 0, 0);              
            macros::LAST_EFFECT_SET_COLOR(fighter,0.0, 0.784, 1.0,);
            macros::BURN_COLOR(fighter, 0.0, 0.784, 1.0, 0.5);
        }
        else {
            let rotation = smash::phx::Vector3f {x:0.0,y:0.0,z: -90.0};
            //EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_jump_aerial"), smash::phx::Hash40::new("waist"), &zero, &rotation, 1.0, &zero, &zero, false, 0, 0, 0);              
            macros::LAST_EFFECT_SET_COLOR(fighter,0.0, 0.784, 1.0,);
            macros::BURN_COLOR(fighter, 0.0, 0.784, 1.0, 0.5);
        }
        WHIFF_CANCEL_INTO_DASH[entry_id] = false;
    }
    if [ 
    *FIGHTER_STATUS_KIND_ESCAPE_AIR 
    ].contains(&status_kind) {
        let stick_degrees = ControlModule::get_stick_angle(fighter.module_accessor).to_degrees();
        let rotation = smash::phx::Vector3f {x:0.0,y:0.0,z: stick_degrees - 90.0};
        macros::BURN_COLOR(fighter, 0.0, 0.784, 1.0, 0.7);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
            //EffectModule::req_on_joint(fighter.module_accessor, smash::phx::Hash40::new("sys_jump_aerial"), smash::phx::Hash40::new("waist"), &zero, &rotation, 1.0, &zero, &zero, false, 0, 0, 0);              
            macros::LAST_EFFECT_SET_COLOR(fighter,0.0, 0.784, 1.0,);            
        }
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        cancel_in_neutral, 
        landing_lag
    );
    }