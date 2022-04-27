use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;
use smash_script::*;
use crate::lightning_02_up_special_callbacks::UP_SPECIAL_ANIMATION;
use crate::lightning_02_up_special_callbacks::ENTRY_ID;
use smash::phx::Hash40;
use smash::hash40;

// Use this for general per-frame fighter-level hooks
#[fighter_frame_callback]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 1);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
        let frame = MotionModule::frame(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        let jump_guard_dash_upspecial_pressed = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0 || (situation_kind == *SITUATION_KIND_GROUND && (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0) || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0);
        let jump_button_pressed = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0;
        let stick_x = ControlModule::get_stick_x;

        
        //CANCEL ON HIT (EXCEPT UP SPECIALS)
        
        
        if ! (fighter_kind == *FIGHTER_KIND_CAPTAIN
            ||fighter_kind == *FIGHTER_KIND_CHROM
            ||fighter_kind == *FIGHTER_KIND_CLOUD
            ||fighter_kind == *FIGHTER_KIND_DAISY
            ||fighter_kind == *FIGHTER_KIND_DEMON
            ||fighter_kind == *FIGHTER_KIND_DIDDY
            ||fighter_kind == *FIGHTER_KIND_DOLLY
            ||fighter_kind == *FIGHTER_KIND_EDGE
            ||fighter_kind == *FIGHTER_KIND_EFLAME
            ||fighter_kind == *FIGHTER_KIND_ELIGHT
            ||fighter_kind == *FIGHTER_KIND_FALCO
            ||fighter_kind == *FIGHTER_KIND_FOX
            ||fighter_kind == *FIGHTER_KIND_GANON
            ||fighter_kind == *FIGHTER_KIND_JACK
            ||fighter_kind == *FIGHTER_KIND_KAMUI
            ||fighter_kind == *FIGHTER_KIND_KOOPA
            ||fighter_kind == *FIGHTER_KIND_LINK
            ||fighter_kind == *FIGHTER_KIND_LITTLEMAC
            ||fighter_kind == *FIGHTER_KIND_MARTH
            ||fighter_kind == *FIGHTER_KIND_LUCINA
            ||fighter_kind == *FIGHTER_KIND_MASTER
            ||fighter_kind == *FIGHTER_KIND_MIIFIGHTER
            ||fighter_kind == *FIGHTER_KIND_MIIGUNNER
            ||fighter_kind == *FIGHTER_KIND_MIISWORDSMAN
            ||fighter_kind == *FIGHTER_KIND_PEACH
            ||fighter_kind == *FIGHTER_KIND_POPO
            ||fighter_kind == *FIGHTER_KIND_NANA


            ||fighter_kind == *FIGHTER_KIND_PALUTENA 
            ||fighter_kind == *FIGHTER_KIND_PIKACHU
            ||fighter_kind == *FIGHTER_KIND_PICHU
            ||fighter_kind == *FIGHTER_KIND_PIT
            ||fighter_kind == *FIGHTER_KIND_PITB
            
            ||fighter_kind == *FIGHTER_KIND_ROY
            ||fighter_kind == *FIGHTER_KIND_ROBOT
            ||fighter_kind == *FIGHTER_KIND_SHEIK
            ||fighter_kind == *FIGHTER_KIND_SHULK
            ||fighter_kind == *FIGHTER_KIND_SIMON
            ||fighter_kind == *FIGHTER_KIND_RICHTER


            ||fighter_kind == *FIGHTER_KIND_SAMUS
            ||fighter_kind == *FIGHTER_KIND_SAMUSD
            ||fighter_kind == *FIGHTER_KIND_TANTAN
            ||fighter_kind == *FIGHTER_KIND_DEMON+1 
            ||fighter_kind == *FIGHTER_KIND_YOUNGLINK
            ||fighter_kind == *FIGHTER_KIND_SONIC

            ||fighter_kind == *FIGHTER_KIND_TOONLINK
            ||fighter_kind == *FIGHTER_KIND_ZELDA
        ){
            
            if !(status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
            //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
            //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3)
            && ! (status_kind == *FIGHTER_STATUS_KIND_THROW)
            {
                if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor){
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
            }  
        }  
        

        //REMOVE INVINCIBILITY ON SHIELD BREAK 
        
        if status_kind == *FIGHTER_STATUS_KIND_SHIELD_BREAK_FALL {
            macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
        }

        //EASIER WAVEDASH CHAINS// 
        if status_kind == *FIGHTER_STATUS_KIND_LANDING && frame >10.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
            
        
        //REMOVE LANDING LAG
        if (status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR || status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL) && frame >1.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
        
        //AIRDASH
        if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
            if frame >= 4.0 {
                
                CancelModule::enable_cancel(fighter.module_accessor);
                
            }
        }
        
        // ROLL??
        if (motion_kind== smash::hash40("escape_f") || motion_kind== smash::hash40("escape_b") ) {

        }

        //SHIMMY
        if status_kind == *FIGHTER_STATUS_KIND_DASH {
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 {
                //PostureModule::reverse_lr(fighter.module_accessor);
            }   
        }

        //DODGES ARE FASTER
        
        //Get airdodge back during free fall
        if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            //if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD != 0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor,*FIGHTER_STATUS_KIND_FALL,true); 
            //}   
        }
        
        //GRAB COMBOS
        if status_kind == *FIGHTER_STATUS_KIND_THROW && AttackModule::is_attack_occur(fighter.module_accessor) {
            
            if ! (fighter_kind == *FIGHTER_KIND_SHULK && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0) { //so shulk's monado arts don't break
                CancelModule::enable_cancel(fighter.module_accessor);
            }
            
        }
        //MOONWALKING
        let speed = smash::phx::Vector3f {x: -1.0, y: 0.0, z: 0.0 };
        let mut stick_x = ControlModule::get_stick_x(fighter.module_accessor);
        stick_x = stick_x * PostureModule::lr(fighter.module_accessor);

        if (motion_kind == smash::hash40("dash") || motion_kind == smash::hash40("turn_dash")) && MotionModule::frame(fighter.module_accessor) <= 3.0 && stick_x <= -0.3 {
            KineticModule::add_speed(fighter.module_accessor,  &speed);
        };
       

        //REWARD PARRIES WITH INVINCIBILITY

        if motion_kind== smash::hash40("just_shield") || motion_kind== smash::hash40("just_shield_off") {
            HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);    
            
        }

        

        //NO JAB CHAINS (CANCEL WITH JUMP/GRAB/DASH)
        if motion_kind== smash::hash40("attack_11")
        || motion_kind== smash::hash40("attack_12")
        || motion_kind== smash::hash40("attack_13")
        || motion_kind== smash::hash40("attack_14")
        || motion_kind== smash::hash40("attack_15")
        || motion_kind== smash::hash40("attack_16")
        || motion_kind== smash::hash40("attack_17")
        || motion_kind== smash::hash40("attack_18")
        || motion_kind== smash::hash40("attack_19")
        || motion_kind== smash::hash40("attack_100")
        || motion_kind== smash::hash40("attack_100_sub")
        || motion_kind== smash::hash40("attack_100_end")
        || motion_kind== smash::hash40("attack_110") {
            
            if AttackModule::is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor) {
                
                //if status_kind == *FIGHTER_STATUS_KIND_ATTACK
                //|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_100
                //|| status_kind == *FIGHTER_TANTAN_STATUS_KIND_ATTACK_COMBO {
                    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    }
                    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_CATCH, true);
                    }
                    if ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0) {
                        if ! fighter_kind == *FIGHTER_KIND_DEMON { //Kazuya is the only one that can't dash after jab
                           CancelModule::enable_cancel(fighter.module_accessor); 
                        }
                        
                    }
                //}  
            } 
        }
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(once_per_fighter_frame);

}