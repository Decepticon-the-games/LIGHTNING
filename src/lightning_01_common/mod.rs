use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;

// Use this for general per-frame fighter-level hooks
#[fighter_frame_callback]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let jump_button_pressed = ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
        let frame = MotionModule::frame(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        let jump_guard_dash_upspecial_pressed = ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0 || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0);
    
        //CANCEL ON HIT (EXCEPT UP SPECIALS)
        if !(status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3)
        {
        
            if ! (fighter_kind == *FIGHTER_KIND_CAPTAIN
                ||fighter_kind == *FIGHTER_KIND_CHROM
                ||fighter_kind == *FIGHTER_KIND_CLOUD
                ||fighter_kind == *FIGHTER_KIND_DIDDY
                ||fighter_kind == *FIGHTER_KIND_EDGE
                ||fighter_kind == *FIGHTER_KIND_EFLAME
                ||fighter_kind == *FIGHTER_KIND_ELIGHT
                ||fighter_kind == *FIGHTER_KIND_FALCO
                ||fighter_kind == *FIGHTER_KIND_FOX
                ||fighter_kind == *FIGHTER_KIND_GANON
                ||fighter_kind == *FIGHTER_KIND_JACK
                ||fighter_kind == *FIGHTER_KIND_KOOPA
                ||fighter_kind == *FIGHTER_KIND_LINK
                ||fighter_kind == *FIGHTER_KIND_LITTLEMAC
                ||fighter_kind == *FIGHTER_KIND_MARTH
                ||fighter_kind == *FIGHTER_KIND_LUCINA
                ||fighter_kind == *FIGHTER_KIND_PALUTENA
                ||fighter_kind == *FIGHTER_KIND_PIT
                ||fighter_kind == *FIGHTER_KIND_PITB
                ||fighter_kind == *FIGHTER_KIND_ROY
                ||fighter_kind == *FIGHTER_KIND_SHULK
                ||fighter_kind == *FIGHTER_KIND_SIMON
                ||fighter_kind == *FIGHTER_KIND_RICHTER
                ||fighter_kind == *FIGHTER_KIND_ZELDA

                
            ){
                if AttackModule:: is_attack_occur(fighter.module_accessor)  &&  ! AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
            }  
        }  

        //BALANCE UP SMASH/SIDE SMASH/ UP TILT
        
        if (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4) && AttackModule:: is_attack_occur(fighter.module_accessor) {
            if jump_guard_dash_upspecial_pressed {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }

        //EASIER WAVEDASH CHAINS// 
        if motion_kind== smash::hash40("landing_light") || motion_kind== smash::hash40("landing_heavy") {
            if frame >= 10.0 && jump_button_pressed {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false);
            }
            
        }
        
        //AIRDASH
        if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
            if frame >= 3.0 {
                
                CancelModule::enable_cancel(fighter.module_accessor);
                
            }
        }
        //RESET AIRDODGE ON HIT EXCEPT UP SPECIAL OF ALL KINDS
       
        if situation_kind == *SITUATION_KIND_AIR {
            if ! (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
                || status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A
                || status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G
                || status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP ){
                if AttackModule::is_attack_occur(fighter.module_accessor) && (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD != 0) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                }   
            }
        }
        
        //Get airdodge back during free fall
        if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD != 0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor,*FIGHTER_STATUS_KIND_ESCAPE_AIR,true); 
            }   
        }
        
        //GRAB COMBOS
        if status_kind == *FIGHTER_STATUS_KIND_THROW && StopModule::is_damage(fighter.module_accessor) {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
        
       

        //REWARD PARRIES WITH INVINCIBILITY

        if motion_kind== smash::hash40("just_shield") || motion_kind== smash::hash40("just_shield_off") {
            HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);    
            
        }

        

        //NO JAB CHAINS (CANCEL WITH JUMP/GRAB/DASH)
        if motion_kind== smash::hash40("attack_11")
        || motion_kind== smash::hash40("attack_12")
        || motion_kind== smash::hash40("attack_13")
        || motion_kind== smash::hash40("attack_100")
        || motion_kind== smash::hash40("attack_100_sub")
        || motion_kind== smash::hash40("attack_100_end") {
            
            if AttackModule::is_attack_occur(fighter.module_accessor) {
                
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK
                || status_kind == *FIGHTER_STATUS_KIND_ATTACK_100 {
                    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    }
                    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_CATCH, true);
                    }
                    if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0 {
                        CancelModule::enable_cancel(fighter.module_accessor);
                    }
                }  
            } 
        }
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(once_per_fighter_frame);
}