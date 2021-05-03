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
        let status_kind = StatusModule::status_kind(module_accessor);
        ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        //Set up bool to make all types of up bs to only be used once in the air no matter what
        //if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
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
        {
            UP_SPECIAL[ENTRY_ID] = true;
        }
        //Reset up b once you're in certain situations
        if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND 
        || StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_RESTRAINT
        || StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_CLIFF
        || StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_WATER
        || StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_LADDER
        || status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_HI_THROW 
        || status_kind == *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW
        || status_kind == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_HI_WALL_JUMP
        || status_kind == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_HI_OVERTAKE
        || status_kind == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_HI_FAIL
        || status_kind == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_HI_HIT
        || status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_CUT
        || status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_PULL
        || status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_THROW
        || status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_PICKUP
        || StopModule::is_damage(module_accessor)
        || StopModule::is_stop(module_accessor)
        || status_kind == *FIGHTER_STATUS_KIND_THROWN
        || status_kind == *FIGHTER_STATUS_KIND_REBIRTH
        || status_kind == *FIGHTER_STATUS_KIND_CLUNG_DIDDY
        || status_kind == *FIGHTER_STATUS_KIND_CLUNG_GANON
        || status_kind == *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN
        || status_kind == *FIGHTER_STATUS_KIND_CATCHED_GANON
        || status_kind == *FIGHTER_STATUS_KIND_MEWTWO_THROWN
        || (fighter_kind == *FIGHTER_KIND_BAYONETTA && status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL)
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