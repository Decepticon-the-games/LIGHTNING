use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::lib::lua_const::*;
use acmd::*;
use smash::hash40;

// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let motion_kind = MotionModule::motion_kind(module_accessor);       
        let frame = MotionModule::frame(module_accessor);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        //---------------------------------------------------------------------------------------------------------
        //Cancel Up Special
        //-----------------------------------------------------------------------------------------------------------
        //-----------------------------------------------------------------------------------------------------------


        //if fighter_kind == *FIGHTER_KIND_BAYONETTA
        //if fighter_kind == *FIGHTER_KIND_BRAVE
        //if fighter_kind == *FIGHTER_KIND_BUDDY
        //if fighter_kind == *FIGHTER_KIND_CAPTAIN
        if fighter_kind == *FIGHTER_KIND_CHROM {
            if motion_kind == smash::hash40("special_hi2") || motion_kind == smash::hash40(("special_air_hi2")) {

                //Cancel into side b

                if  frame >= 31.0 && (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0) {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
                }

                //Cancel into up air

        
            }
             
        }
        
        if fighter_kind == *FIGHTER_KIND_CLOUD {
            if motion_kind == smash::hash40("special_hi") {

                //set clout to wait if up b connects and up b is rapidly pressed again (stylistic effect)

                if frame == 6.0 {
                    if AttackModule::is_attack_occur(module_accessor) {
                        if  (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0) {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                        }
                    }
                    //if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    //    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
                    //}
                        
                    
                }
                if frame == 10.0 {
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                        //if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0){
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        //}
                    }
                
                    //if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    //    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
                    //}
                        
                    
                }
                if frame >= 11.0 {
                    CancelModule::enable_cancel(module_accessor);
                    
                }
            }
        }
        //if fighter_kind == *FIGHTER_KIND_DEDEDE
        if fighter_kind == *FIGHTER_KIND_DIDDY {

        }
        //if fighter_kind == *FIGHTER_KIND_DOLLY
        //if fighter_kind == *FIGHTER_KIND_DONKEY
        //if fighter_kind == *FIGHTER_KIND_DUCKHUNT
        //if fighter_kind == *FIGHTER_KIND_EDGE
        //if fighter_kind == *FIGHTER_KIND_EFLAME
        //if fighter_kind == *FIGHTER_KIND_ELIGHT
        //if fighter_kind == *FIGHTER_KIND_FALCO
        //if fighter_kind == *FIGHTER_KIND_FOX
        //if fighter_kind == *FIGHTER_KIND_GAMEWATCH
        //if fighter_kind == *FIGHTER_KIND_GANON
        //if fighter_kind == *FIGHTER_KIND_GAOGAEN
        //if fighter_kind == *FIGHTER_KIND_GEKKOUGA
        //if fighter_kind == *FIGHTER_KIND_IKE
        //if fighter_kind == *FIGHTER_KIND_INKLING 
        //if fighter_kind == *FIGHTER_KIND_JACK
        //if fighter_kind == *FIGHTER_KIND_KAMUI
        //if fighter_kind == *FIGHTER_KIND_KEN
        //if fighter_kind == *FIGHTER_KIND_KIRBY
        if fighter_kind == *FIGHTER_KIND_KOOPA {

            //Cancel into nair


            //Cancel into up air

            
        }
        //if fighter_kind == *FIGHTER_KIND_KOOPAJR
        //if fighter_kind == *FIGHTER_KIND_KROOL
        //if fighter_kind == *FIGHTER_KIND_LINK
        //if fighter_kind == *FIGHTER_KIND_LITTLEMAC
        //if fighter_kind == *FIGHTER_KIND_LIZARDON
        //if fighter_kind == *FIGHTER_KIND_LUCARIO
        //if fighter_kind == *FIGHTER_KIND_LUCAS
        //if fighter_kind == *FIGHTER_KIND_LUCINA
        //if fighter_kind == *FIGHTER_KIND_LUIGI
        //if fighter_kind == *FIGHTER_KIND_MARIO
        //if fighter_kind == *FIGHTER_KIND_MARIOD
        //if fighter_kind == *FIGHTER_KIND_MARTH
        //if fighter_kind == *FIGHTER_KIND_MASTER
        //if fighter_kind == *FIGHTER_KIND_METAKNIGHT
        //if fighter_kind == *FIGHTER_KIND_MEWTWO
        //if fighter_kind == *FIGHTER_KIND_MIIFIGHTER
        //if fighter_kind == *FIGHTER_KIND_MIIGUNNER
        //if fighter_kind == *FIGHTER_KIND_MIISWORDSMAN
        //if fighter_kind == *FIGHTER_KIND_MURABITO
        //if fighter_kind == *FIGHTER_KIND_NANA
        //if fighter_kind == *FIGHTER_KIND_NESS
        //if fighter_kind == *FIGHTER_KIND_PACKUN
        if fighter_kind == *FIGHTER_KIND_PACMAN {
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {

                //Cancel into fruit throw

                
            }
             
        }
        //if fighter_kind == *FIGHTER_KIND_PALUTENA
        //if fighter_kind == *FIGHTER_KIND_PEACH
        //if fighter_kind == *FIGHTER_KIND_PFUSHIGISOU
        //if fighter_kind == *FIGHTER_KIND_PICHU
        //if fighter_kind == *FIGHTER_KIND_PICKEL
        //if fighter_kind == *FIGHTER_KIND_PIKACHU
        //if fighter_kind == *FIGHTER_KIND_PIKMIN
        //if fighter_kind == *FIGHTER_KIND_PIT
        //if fighter_kind == *FIGHTER_KIND_PITB
        //if fighter_kind == *FIGHTER_KIND_PLIZARDON
        //if fighter_kind == *FIGHTER_KIND_POPO
        //if fighter_kind == *FIGHTER_KIND_PURIN
        //if fighter_kind == *FIGHTER_KIND_PZENIGAME
        if fighter_kind == *FIGHTER_KIND_REFLET {
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI{
                
                //Cancel into side b

                if  (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0) {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
                }
            }
        }
        //if fighter_kind == *FIGHTER_KIND_RICHTER
        //if fighter_kind == *FIGHTER_KIND_RIDLEY
        //if fighter_kind == *FIGHTER_KIND_ROBOT
        //if fighter_kind == *FIGHTER_KIND_ROCKMAN
        //if fighter_kind == *FIGHTER_KIND_ROSETTA
        if fighter_kind == *FIGHTER_KIND_ROY {
            if motion_kind == smash::hash40("special_hi") {

                //Cancel into airdodge 

                if frame == 10.0 || frame == 19.0 {
                    if ! AttackModule::is_attack_occur(module_accessor){
                       if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        } 
                    }  
                }               
            
                //Cancel into side b

                if  (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0) {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
                }
                
                //Cancel into up air

                
            }
             
        }        
        //if fighter_kind == *FIGHTER_KIND_RYU
        if fighter_kind == *FIGHTER_KIND_SAMUS || fighter_kind == *FIGHTER_KIND_SAMUSD {
            if motion_kind == smash::hash40("special_hi") || motion_kind == smash::hash40("special_hi") {
                if frame == 20.0 {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
        //if fighter_kind == *FIGHTER_KIND_SHEIK
        //if fighter_kind == *FIGHTER_KIND_SHIZUE
        //if fighter_kind == *FIGHTER_KIND_SHULK
        //if fighter_kind == *FIGHTER_KIND_SIMON
        //if fighter_kind == *FIGHTER_KIND_SNAKE
        //if fighter_kind == *FIGHTER_KIND_SONIC
        if fighter_kind == *FIGHTER_KIND_SZEROSUIT {
            if motion_kind == smash::hash40("special_hi") {
                if frame == 6.0 || frame == 10.0 {
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                        //if ! (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0){
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                        //}
                    }
                    
                        
                    
                }
                
            }
        }
        //if fighter_kind == *FIGHTER_KIND_TANTAN
        if fighter_kind == *FIGHTER_KIND_TOONLINK {
            if motion_kind == smash::hash40("special_air_hi") {

                //Cancel into up air

                
            }
             
        }
        //if fighter_kind == *FIGHTER_KIND_WARIO
        //if fighter_kind == *FIGHTER_KIND_WIIFIT
        if fighter_kind == *FIGHTER_KIND_WOLF {

            //Cancel into dair
  
        }
        //if fighter_kind == *FIGHTER_KIND_YOSHI
        //if fighter_kind == *FIGHTER_KIND_YOUNGLINK
        //if fighter_kind == *FIGHTER_KIND_ZELDA

       
         
    }
}


pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
}