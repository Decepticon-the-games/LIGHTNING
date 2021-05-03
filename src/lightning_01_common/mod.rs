use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;


// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        //let fighter_kind = utility::get_kind(module_accessor);
        let status_kind = StatusModule::status_kind(module_accessor);
        let situation_kind = StatusModule::situation_kind(module_accessor);
        let jump_button_pressed = ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP);
        let motion_kind = MotionModule::motion_kind(module_accessor);       
        let frame = MotionModule::frame(module_accessor);
        //let jump_dash_pressed = (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0);
        
                 
        
        //EASIER WAVEDASH CHAINS// 
        if motion_kind== smash::hash40("landing_light") || motion_kind== smash::hash40("landing_heavy") {
            if frame >= 10.0 && jump_button_pressed {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false);
            }
            
        }
        
        //AIRDASH
        if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
            if frame >= 4.0 {
                
                CancelModule::enable_cancel(module_accessor);
                
            }
        }
        //RESET AIRDODGE ON HIT EXCEPT UP SPECIAL OF ALL KINDS
       
        //if situation_kind == *SITUATION_KIND_AIR 
        if ! (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
        || status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI
        || status_kind == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2
        || status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2
        || status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3
        || status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4
        || status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A
        || status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G
        || status_kind == *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH
        || status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH
        || status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_FALL
        || status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP
        || status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH
        || status_kind == *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_BOUND
        || status_kind == *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD
        || status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD
        || status_kind == *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_FAIL
        || status_kind == *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP
        || status_kind == *FIGHTER_SHULK_STATUS_KIND_SPECIAL_HI_ADD
        || status_kind == *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_CUT
        || status_kind == *FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_RUSH
        || status_kind == *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_HI_HOLD
        || status_kind == *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_HI_JUMP
        || status_kind == *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_JUMP
        || status_kind == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_HIT
        || status_kind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_FALL
        || status_kind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP
        || status_kind == *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH
        || status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_RUSH
        || status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL
        || status_kind == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD
        || status_kind == *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_DROP
        || status_kind == *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_FALL
        || status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_AGAIN
        || status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_FALL
        || status_kind == *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP
        || status_kind == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_MOVE
        || status_kind == *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_HANG
        || status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP
        || status_kind == *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR
        || status_kind == *FIGHTER_WARIO_STATUS_KIND_SPECIAL_HI_JUMP
        || status_kind == *FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_BOUND
        || status_kind == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2_FALL
        || status_kind == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_JUMP
        || status_kind == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_TURN
        || status_kind == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER
        || status_kind == *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_BOUND
        || status_kind == *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_ROT
        || status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_START
        || status_kind == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_AGAIN
        || status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK
        || status_kind == *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_LOOP
        || status_kind == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_FALL
        || status_kind == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_JUMP
        || status_kind == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_WAIT
        || status_kind == *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_WAIT
        || status_kind == *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_FAIL
        || status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND
        || status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_LANDING
        || status_kind == *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_HI_JUMP
        || status_kind == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE
        || status_kind == *FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_FLY
        || status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING
        || status_kind == *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH_END
        || status_kind == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_FALL
        || status_kind == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_LOOP
        || status_kind == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_TURN
        || status_kind == *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_FALL
        || status_kind == *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_JUMP
        || status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_FALL
        || status_kind == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH
        || status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_REFLECT
        || status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP
        || status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END
        || status_kind == *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_PARTNER
        || status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_HOVER
        || status_kind == *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_ATTACK
        || status_kind == *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP
        || status_kind == *FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_HI_JUMP
        || status_kind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND
        || status_kind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_LANDING
        || status_kind == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_BOUND
        || status_kind == *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_HI_LOOP
        || status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_SHOOT
        || status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END
        || status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_LANDING
        || status_kind == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND
        || status_kind == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_REFLECT
        || status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_FLAP
        || status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_TURN
        || status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_WAIT
        || status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END
        || status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_LANDING
        || status_kind == *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP_PRE
        || status_kind == *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND
        || status_kind == *FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_RUSH_END
        || status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP
        || status_kind == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE
        || status_kind == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_LANDING
        || status_kind == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_HIT_CEIL
        || status_kind == *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH_END
        || status_kind == *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_HI_FALL
        || status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ATTACK
        || status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ESCAPE
        || status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP
        || status_kind == *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING
        || status_kind == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_GLIDING
        || status_kind == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_LANDING
        || status_kind == *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_LANDING
        || status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_LANDING
        || status_kind == *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR_END
        || status_kind == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_FALL_ROLL
        || status_kind == *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_HI_CLOSE
        || status_kind == *FIGHTER_GAOGAEN_REVENGE_STATUS_KIND_SPECIAL_HI
        || status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_LANDING
        || status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_START
        || status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP
        || status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI2_JUMP
        || status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH
        || status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH
        || status_kind == *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_CLIFF_COMP
        || status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B
        || status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_F      
        || status_kind == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END
        || status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_LANDING
        || status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI
        || status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_LW
        || status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_STOP_CEIL
        || status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_STOP_WALL
        || status_kind == *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_ITEM_THROW
        || status_kind == *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR_REACH
        ){
            if AttackModule::is_attack_occur(module_accessor) && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD){
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
            }
            
        }
        // Get airdodge back during free fall
        if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
               StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_ESCAPE_AIR,true); 
            }
            
        }
        //GRAB COMBOS
        //if status_kind == *FIGHTER_STATUS_KIND_THROW && StopModule::is_damage(module_accessor) {
        //    CancelModule::enable_cancel(module_accessor);
        //}

        //REWARD PERFECT WAVEDASHES WITH INVINCIBILITY

        //if (status_kind == *FIGHTER_STATUS_KIND_JUMP && frame == 1.0 {

        //    if (motion_kind== smash::hash40("landing_light") || motion_kind== smash::hash40("landing_heavy"))
        //    && StatusModule::prev_status_kind(module_accessor, 0) == FIGHTER_STATUS_KIND_ESCAPE_AIR 
        //    && StatusModule::prev_status_kind(module_accessor, 1) == FIGHTER_STATUS_KIND_JUMP //&& frame == 1.0 )
        //    {
        //        HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);    
        //    }
        //}

        

        //NO JAB CHAINS (CANCEL WITH JUMP/GRAB/DASH)
        if motion_kind== smash::hash40("attack_11")
        || motion_kind== smash::hash40("attack_12")
        || motion_kind== smash::hash40("attack_13")
        || motion_kind== smash::hash40("attack_100")
        || motion_kind== smash::hash40("attack_100_sub")
        || motion_kind== smash::hash40("attack_100_end") {
            
            if AttackModule::is_attack_occur(module_accessor) {
                
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK
                || status_kind == *FIGHTER_STATUS_KIND_ATTACK_100 {
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    }
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_CATCH, true);
                    }
                    if ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0 {
                        CancelModule::enable_cancel(module_accessor);
                    }
                }  
            } 
        }
    }
}

// Use this for general per-frame weapon-level hooks
// pub fn once_per_weapon_frame(fighter_base : &mut L2CFighterBase) {
//     unsafe {
//         let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
//         let frame = smash::app::lua_bind::frame as i32;

//         if frame % 10 == 0 {
//             println!("[Weapon Hook] Frame : {}", frame);
//         }
//     }
// }

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
    //acmd::add_custom_weapon_hooks!(once_per_weapon_frame);
    //skyline::install_hook!(set_rebound_hook);
    //skyline::install_hook!(generate_article_hook);
    //skyline::install_hook!(clear_all_hook);
}