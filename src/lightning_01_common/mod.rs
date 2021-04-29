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
       
        if situation_kind == *SITUATION_KIND_AIR 
        && ! (StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI
        || StatusModule::status_kind(module_accessor) == *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2
        || StatusModule::status_kind(module_accessor) == *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_3
        || StatusModule::status_kind(module_accessor) == *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_4
        || StatusModule::status_kind(module_accessor) == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI
        || StatusModule::status_kind(module_accessor) == *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2
        || StatusModule::status_kind(module_accessor) == *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_3
        || StatusModule::status_kind(module_accessor) == *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_4
        || StatusModule::status_kind(module_accessor) == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2
        || StatusModule::status_kind(module_accessor) == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2
        || StatusModule::status_kind(module_accessor) == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3
        || StatusModule::status_kind(module_accessor) == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4
        || StatusModule::status_kind(module_accessor) == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A
        || StatusModule::status_kind(module_accessor) == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G
        || StatusModule::status_kind(module_accessor) == *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2
        || StatusModule::status_kind(module_accessor) == *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3
        || StatusModule::status_kind(module_accessor) == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH
        || StatusModule::status_kind(module_accessor) == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_CUT
        || StatusModule::status_kind(module_accessor) == *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3
        || StatusModule::status_kind(module_accessor) == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH
        || StatusModule::status_kind(module_accessor) == *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2
        || StatusModule::status_kind(module_accessor) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_FALL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH
        || StatusModule::status_kind(module_accessor) == *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_BOUND
        || StatusModule::status_kind(module_accessor) == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_PULL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD
        || StatusModule::status_kind(module_accessor) == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD
        || StatusModule::status_kind(module_accessor) == *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_FAIL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_SHULK_STATUS_KIND_SPECIAL_HI_ADD
        || StatusModule::status_kind(module_accessor) == *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_CUT
        || StatusModule::status_kind(module_accessor) == *FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_RUSH
        || StatusModule::status_kind(module_accessor) == *WEAPON_JACK_DOYLE_STATUS_KIND_SPECIAL_HI
        || StatusModule::status_kind(module_accessor) == *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_HI_HOLD
        || StatusModule::status_kind(module_accessor) == *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_HI_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_HIT
        || StatusModule::status_kind(module_accessor) == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_FALL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH
        || StatusModule::status_kind(module_accessor) == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_RUSH
        || StatusModule::status_kind(module_accessor) == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_THROW
        || StatusModule::status_kind(module_accessor) == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_HOLD
        || StatusModule::status_kind(module_accessor) == *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_DROP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_FALL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_HI_HIT
        || StatusModule::status_kind(module_accessor) == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_AGAIN
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_FALL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_MOVE
        || StatusModule::status_kind(module_accessor) == *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_HANG
        || StatusModule::status_kind(module_accessor) == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR
        || StatusModule::status_kind(module_accessor) == *FIGHTER_WARIO_STATUS_KIND_SPECIAL_HI_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_BOUND
        || StatusModule::status_kind(module_accessor) == *WEAPON_IKE_SWORD_STATUS_KIND_SPECIAL_HI_2
        || StatusModule::status_kind(module_accessor) == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2_FALL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_TURN
        || StatusModule::status_kind(module_accessor) == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER
        || StatusModule::status_kind(module_accessor) == *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_BOUND
        || StatusModule::status_kind(module_accessor) == *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING
        || StatusModule::status_kind(module_accessor) == *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW
        || StatusModule::status_kind(module_accessor) == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_ROT
        || StatusModule::status_kind(module_accessor) == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_PICKUP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_START
        || StatusModule::status_kind(module_accessor) == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_AGAIN
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_HI_FAIL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_LOOP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_FALL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_WAIT
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_WAIT
        || StatusModule::status_kind(module_accessor) == *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_FAIL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND
        || StatusModule::status_kind(module_accessor) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_LANDING
        || StatusModule::status_kind(module_accessor) == *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_HI_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE
        || StatusModule::status_kind(module_accessor) == *FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_FLY
        || StatusModule::status_kind(module_accessor) == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING
        || StatusModule::status_kind(module_accessor) == *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_FALL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_LOOP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_TURN
        || StatusModule::status_kind(module_accessor) == *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_FALL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_FALL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH
        || StatusModule::status_kind(module_accessor) == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_ATTACK
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIENEMYF_STATUS_KIND_SPECIAL_HI2_2
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIENEMYF_STATUS_KIND_SPECIAL_HI2_3
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIENEMYF_STATUS_KIND_SPECIAL_HI2_4
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_REFLECT
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_PARTNER
        || StatusModule::status_kind(module_accessor) == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_HOVER
        || StatusModule::status_kind(module_accessor) == *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_ATTACK
        || StatusModule::status_kind(module_accessor) == *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_HI_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_HI_CLING
        || StatusModule::status_kind(module_accessor) == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_HI_THROW
        || StatusModule::status_kind(module_accessor) == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND
        || StatusModule::status_kind(module_accessor) == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_LANDING
        || StatusModule::status_kind(module_accessor) == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_BOUND
        || StatusModule::status_kind(module_accessor) == *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_HI_LOOP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_SHOOT
        || StatusModule::status_kind(module_accessor) == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_LANDING
        || StatusModule::status_kind(module_accessor) == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND
        || StatusModule::status_kind(module_accessor) == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_REFLECT
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI1_2
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI1_3
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI1_4
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_FLAP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_TURN
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_WAIT
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_LANDING
        || StatusModule::status_kind(module_accessor) == *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP_PRE
        || StatusModule::status_kind(module_accessor) == *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND
        || StatusModule::status_kind(module_accessor) == *FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_RUSH_END
        || StatusModule::status_kind(module_accessor) == *WEAPON_JACK_DOYLE_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2_LANDING
        || StatusModule::status_kind(module_accessor) == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE
        || StatusModule::status_kind(module_accessor) == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_LANDING
        || StatusModule::status_kind(module_accessor) == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_HIT_CEIL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_HI_FALL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ATTACK
        || StatusModule::status_kind(module_accessor) == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ESCAPE
        || StatusModule::status_kind(module_accessor) == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIENEMYF_STATUS_KIND_SPECIAL_HI3_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIENEMYS_STATUS_KIND_SPECIAL_HI1_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIENEMYS_STATUS_KIND_SPECIAL_HI3_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_GLIDING
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_LANDING
        || StatusModule::status_kind(module_accessor) == *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_LANDING
        || StatusModule::status_kind(module_accessor) == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_LANDING
        || StatusModule::status_kind(module_accessor) == *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_FALL_ROLL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_HI_CLOSE
        || StatusModule::status_kind(module_accessor) == *FIGHTER_GAOGAEN_REVENGE_STATUS_KIND_SPECIAL_HI
        || StatusModule::status_kind(module_accessor) == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_LANDING
        || StatusModule::status_kind(module_accessor) == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_START
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_HI_OVERTAKE
        || StatusModule::status_kind(module_accessor) == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIENEMYG_STATUS_KIND_SPECIAL_HI1_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIENEMYG_STATUS_KIND_SPECIAL_HI2_FAIL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIENEMYG_STATUS_KIND_SPECIAL_HI3_RUSH
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIENEMYS_STATUS_KIND_SPECIAL_HI1_HOLD
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIENEMYS_STATUS_KIND_SPECIAL_HI2_RUSH
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIENEMYS_STATUS_KIND_SPECIAL_HI3_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIENEMYS_STATUS_KIND_SPECIAL_HI3_LOOP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI2_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI2_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH
        || StatusModule::status_kind(module_accessor) == *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_CLIFF_COMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B
        || StatusModule::status_kind(module_accessor) == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_F
        || StatusModule::status_kind(module_accessor) == *WEAPON_ROSETTA_TICO_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_HI_WALL_JUMP
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MIIENEMYS_STATUS_KIND_SPECIAL_HI2_BOUND
        || StatusModule::status_kind(module_accessor) == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_LANDING
        || StatusModule::status_kind(module_accessor) == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI
        || StatusModule::status_kind(module_accessor) == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_LW
        || StatusModule::status_kind(module_accessor) == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_STOP_CEIL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_STOP_WALL
        || StatusModule::status_kind(module_accessor) == *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_ITEM_THROW
        || StatusModule::status_kind(module_accessor) == *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR_REACH
        || StatusModule::status_kind(module_accessor) == *WEAPON_INKLING_SQUID_STATUS_KIND_SPECIAL_HI_ROT
        || StatusModule::status_kind(module_accessor) == *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_HI_SET
        || StatusModule::status_kind(module_accessor) == *WEAPON_ROSETTA_TICO_STATUS_KIND_SPECIAL_HI_JUMP
        ){
            if AttackModule::is_attack_occur(module_accessor) && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD){
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
            
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