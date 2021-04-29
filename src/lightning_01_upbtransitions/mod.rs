use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use acmd;

static mut UP_SPECIAL : [bool; 8] = [false; 8];
static mut ENTRY_ID : usize = 0;
pub fn set_bool(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        //Set up bool to make all types of up bs to only be used once in the air no matter what
        if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR
        && (StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI
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
            UP_SPECIAL[ENTRY_ID] = true;
        }
        //Reset up b once you're in certain situations
        if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND 
        || StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_RESTRAINT
        || StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_CLIFF
        || StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_WATER
        || StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_LADDER
        || StatusModule::status_kind(module_accessor) == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_HI_CLING 
        || StatusModule::status_kind(module_accessor) == *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING
        || StopModule::is_damage(module_accessor)
        || StopModule::is_hit(module_accessor)
        || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_THROWN
        || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH
        || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLUNG_DIDDY
        || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLUNG_GANON
        || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN
        || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CATCHED_GANON
        || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_MEWTWO_THROWN
        || (fighter_kind == *FIGHTER_KIND_BAYONETTA && StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_JUMP_AERIAL)
        {
            UP_SPECIAL[ENTRY_ID] = false;
        }
    
    }
}

#[skyline::hook(replace=smash::app::lua_bind::WorkModule::is_enable_transition_term)]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let ret = original!()(module_accessor,term);
    
    ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if UP_SPECIAL[ENTRY_ID] {
        //Use up b only once in the air 
        if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI {
            return false;
        }
        else {
            return ret;
        }
    }
    else {
        return ret;
    }  
}





    


pub fn install() {
    acmd::add_custom_hooks!(set_bool);
    skyline::install_hook!(is_enable_transition_term_replace);  
}