
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::lib::lua_const::*;
use smashline::*;

// Use this for general per-frame fighter-level hooks
#[fighter_frame_callback]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = smash::app::utility::get_kind(fighter.module_accessor);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        
        //DISABLE UP SPECIAL/PUMMEL INFLICTION CANCEL
        if ! (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
        && ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        {

            //FIX/ENHANCE MOVES
            //-------------------------
            //Mario
            //Cloud
            //--------------------------------------------------------------------------------------------------------
            //Fix Side special/Side Smash
            if fighter_kind == *FIGHTER_KIND_CLOUD
            && (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
            ||  status_kind == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2
            ||  status_kind == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3)
            ||  status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
                //Side Special
                if status_kind == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3 {

                    if MotionModule::frame(fighter.module_accessor) >=25.0  {                       
                        if AttackModule:: is_attack_occur(fighter.module_accessor){
                            CancelModule::enable_cancel(fighter.module_accessor);
                        }
                    }
                }
                //Side Smash
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
                    if MotionModule::frame(fighter.module_accessor) >=28.0  {                       
                        if AttackModule:: is_attack_occur(fighter.module_accessor){
                            CancelModule::enable_cancel(fighter.module_accessor);
                        }
                    }
                }
            }
            //Donkey Kong
            //--------------------------------------------------------------------------------------------------------
            //Fix Up Special

            //Link
            //--------------------------------------------------------------------------------------------------------
                //Fix Up Smash/Side Smash
            else if fighter_kind == *FIGHTER_KIND_LINK
            && (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4   
            || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4) {
                //Up Smash
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {

                    if MotionModule::frame(fighter.module_accessor) >= 40.0 {
                        if AttackModule:: is_attack_occur(fighter.module_accessor){
                            CancelModule::enable_cancel(fighter.module_accessor);
                        }
                    }
                }
                //Side Smash
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {

                    if MotionModule::frame(fighter.module_accessor) >= 36.0 {
                        if AttackModule:: is_attack_occur(fighter.module_accessor){
                            CancelModule::enable_cancel(fighter.module_accessor);
                        }
                    }
                }
            }
            //Samus/Dark Samus
            //-----------------------------------------------------------------------------------------------------------   
                // Up air cancel Up Special
            else if fighter_kind == *FIGHTER_KIND_SAMUS || fighter_kind == *FIGHTER_KIND_SAMUSD
            && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
                
                if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
                    
                    if MotionModule::frame(fighter.module_accessor) == 15.0 {
                        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
                        }
                    }
                }
            }
            //Yoshi
            //----------------------------------------------------------------------------------------------
            //Kirby
            //----------------------------------------------------------------------------------------------
            //Pikachu
            //----------------------------------------
            //Luigi
            //Ness
            //Captain Falcon
            //Jigglypuff
            //Peach/Daisy
            //Bowser
            //-----------------------------------------------------------------------------------------------
                //Fix Up Special/Attack cancel Up Special
            else if fighter_kind == *FIGHTER_KIND_KOOPA
            && status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G
            || status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A{
                //Attack cancel Up Special
                if status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G {

                    if MotionModule::frame(fighter.module_accessor) >=37.0{
                        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK){
                            CancelModule::enable_cancel(fighter.module_accessor);
                        }
                    }

                //Fix Up Special 
                    if MotionModule::frame(fighter.module_accessor) >=38.0 {
                        if AttackModule:: is_attack_occur(fighter.module_accessor){
                            CancelModule::enable_cancel(fighter.module_accessor);
                        }
                    }
                }
                if status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A {

                    if MotionModule::frame(fighter.module_accessor) >=45.0 {
                        if AttackModule:: is_attack_occur(fighter.module_accessor){
                            CancelModule::enable_cancel(fighter.module_accessor);
                        }
                    }
                }
            }
            //Ice Climbers
            //Shiek
            //Zelda
            //Dr.Mario
            //Pichu
            //Falco

            //Fix Fire Emblem Side Specials
            //------------------------------------------------------------------------------------------------------
            //Marth/Lucina
            else if fighter_kind == *FIGHTER_KIND_MARTH || fighter_kind == *FIGHTER_KIND_LUCINA
            && (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
            || status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2
            || status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3
            ||status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4) {

                if status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4 {
            
                    if AttackModule:: is_attack_occur(fighter.module_accessor) {
                    CancelModule::enable_cancel(fighter.module_accessor);
                } 
                }
            }
            //Roy/Chrom
            else if fighter_kind == *FIGHTER_KIND_ROY || fighter_kind == *FIGHTER_KIND_CHROM
            && (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
            || status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2
            || status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3
            ||status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4) {

                if status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4 {
            
                    if AttackModule:: is_attack_occur(fighter.module_accessor) {
                    CancelModule::enable_cancel(fighter.module_accessor);
                }                
                }
            }
            //Young Link
            //Ganondorf
            //Mewtwo
            //Mr.Game & Watch
            //Meta Knight
            //Pit
            //Dark Pit

            //Zero Suit Samus
            //--------------------------------------------------------------------------------------------------------------------
            else if fighter_kind == *FIGHTER_KIND_SZEROSUIT
            && (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4
            || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4) {

                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
            
                    if MotionModule::frame(fighter.module_accessor) >=28.0 {
                        if AttackModule:: is_attack_occur(fighter.module_accessor){
                            CancelModule::enable_cancel(fighter.module_accessor);
                        }
                    } 
                                  
                }
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
            
                    if MotionModule::frame(fighter.module_accessor) >=26.0 {
                        if AttackModule:: is_attack_occur(fighter.module_accessor){
                            CancelModule::enable_cancel(fighter.module_accessor);
                        }
                    } 
                                  
                }
            }
            //Wario
            //Snake
            //Ike
            //Squirtle
            //Ivysaur
            //Charizard
            //Diddy Kong
            //Lucas
            //Sonic
            //King Dedede
            //Olimar
            //Lucario
            //R.O.B.
            //Toon Link
            //Wolf
            //Villager
            //Mega Man
            //Wii Fit Trainer
            //Rosalina and Luma
            //Little Mac
            else if fighter_kind == *FIGHTER_KIND_LITTLEMAC
            && status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {

                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
                    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
                        
                        if MotionModule::frame(fighter.module_accessor) >=7.0 {
                            if AttackModule:: is_attack_occur(fighter.module_accessor){
                                CancelModule::enable_cancel(fighter.module_accessor);
                            }
                        }
                    }
                }
            }
            //Greninja
            //Palutena
            //Pac-Man
            //Robin
            //Shulk
            else if fighter_kind == *FIGHTER_KIND_SHULK
            && (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4
            ||  status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4) {

                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
                    
                    if MotionModule::frame(fighter.module_accessor) >=30.0 {
                        if AttackModule:: is_attack_occur(fighter.module_accessor){
                            CancelModule::enable_cancel(fighter.module_accessor);
                        }
                    }
                }
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
                    
                    if MotionModule::frame(fighter.module_accessor) >=23.0 {
                        if AttackModule:: is_attack_occur(fighter.module_accessor){
                            CancelModule::enable_cancel(fighter.module_accessor);
                        }
                    }
                }
            }
            //Bowser Jr.
            //Duck Hunt
            //Ryu
            //Ken
            //Cloud
            //Corrin
            //Bayonetta
            //Inkling
            //Ridley
            //Simon
            //Richter
            //King K.Rool
            //Isabelle
            //Incineroar
            //Piranha Plant
            //Joker
            else if fighter_kind == *FIGHTER_KIND_JACK 
            && status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
                    if MotionModule::frame(fighter.module_accessor) >=23.0 {
                        if AttackModule:: is_attack_occur(fighter.module_accessor){
                            CancelModule::enable_cancel(fighter.module_accessor);
                        }
                    }
                }
            }
            //Hero
            //Banjo & Kazooie
            //Terry
            //Byleth
            //MinMin
            //Steve
            //Sephiroth
            //Mii Brawler
            //Mii Swordfighter
            //Mii Gunner
                         
            //------------------------------------------------------------------------------------------------ 
                //Cancel On Hit
            //-----------------------------------------------------------------------------------------------
            //else if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
            //|| AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD){
            //    CancelModule::enable_cancel(fighter.module_accessor);
                
            //}
            else if AttackModule:: is_attack_occur(fighter.module_accessor){
                CancelModule::enable_cancel(fighter.module_accessor);
            }
            

            
            
        }
        //Airdodge cancel Up special cancel in neutral
        //if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
        //&& situation_kind == *SITUATION_KIND_AIR {
        //            
        //    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD){
        //        StatusModule::change_status_request_from_script(fighter.module_accessor,*FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
        //    }   
        //}
        
        //Airdodge-cancel/Jump-cancel attacks/specials (without chaining up special in the air)
        //-------------------------------------------------------------------------------------------------
        

        
            

        //NO JAB CHAINS (CANCEL WITH JUMP/GRAB)
        if MotionModule::motion_kind(fighter.module_accessor)== smash::hash40("attack_11")
        || MotionModule::motion_kind(fighter.module_accessor)== smash::hash40("attack_12")
        || MotionModule::motion_kind(fighter.module_accessor)== smash::hash40("attack_13")
        || MotionModule::motion_kind(fighter.module_accessor)== smash::hash40("attack_100")
        || MotionModule::motion_kind(fighter.module_accessor)== smash::hash40("attack_100_sub")
        || MotionModule::motion_kind(fighter.module_accessor)== smash::hash40("attack_100_end") {
            
            if AttackModule::is_attack_occur(fighter.module_accessor) {
                
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK
                || status_kind == *FIGHTER_STATUS_KIND_ATTACK_100 {
                    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                        change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    }
                    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
                        change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_CATCH, true);
                    }
                }  
            } 
        }
    }                                      
}
// Auto-Combo



//Jump Cancel Grab

#[skyline::hook(replace=smash::app::lua_bind::GrabModule::set_rebound)]
pub fn set_rebound_hook(module_accessor: &mut smash::app::BattleObjectModuleAccessor, CanCatchRebound: bool) {
    unsafe  {
        if MotionModule::motion_kind(module_accessor)== smash::hash40("catch")
        || MotionModule::motion_kind(module_accessor)== smash::hash40("catch_dash")
        || MotionModule::motion_kind(module_accessor)== smash::hash40("catch_turn") {
            //set another boolean true that decides for cancelling move
            if CanCatchRebound == true {
               if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);        
                } 
            }
            
        }
        //reset ur previous bool
        original!()(module_accessor, CanCatchRebound);
    }
}
#[skyline::hook(replace=smash::app::lua_bind::AttackModule::clear_all)]
unsafe fn clear_all_hook(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    
    //if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
    //    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
    //        //StatusModule::change_status_request_from_script(fighter.module_accessor,*FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
    //       CancelModule::enable_cancel(fighter.module_accessor);
    //   }  
    //    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
    //        //StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
    //        CancelModule::enable_cancel(fighter.module_accessor);
    //    } 
    //}   

    //if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
    //        
    //    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
    //       StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
    //        
    //        
    //    }
    //}

    //if ! StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
       CancelModule::enable_cancel(module_accessor);
    //} 
        
    
    original!()(module_accessor)
    
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(once_per_fighter_frame);
    //skyline::install_hook!(set_rebound_hook);
    //skyline::install_hook!(clear_all_hook);
}