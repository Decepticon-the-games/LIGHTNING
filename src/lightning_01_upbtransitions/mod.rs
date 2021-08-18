use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;

pub static mut UP_SPECIAL : [bool; 8] = [false; 8];
static mut ENTRY_ID : usize = 0;

#[fighter_frame_callback]
pub fn set_bool(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let status_kind = StatusModule::status_kind(module_accessor);
        ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        //Set up bool to make all types of aerial up bs to only be used once no matter what
        //if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR
        if (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
        || status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A
        || status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G
        || status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP
        || status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI2_ATTACK
        || status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_JUMP
        || status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH
        || status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI3_HOLD
        || status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI2_JUMP
        || status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH
        ) && MotionModule::frame(module_accessor) == 2.0
        {
            UP_SPECIAL[ENTRY_ID] = true;
        }
        //Reset up b once you're in certain situations
        if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND 
        || StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_RESTRAINT
        || StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_CLIFF
        || StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_WATER
        || StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_LADDER
        || (fighter_kind == *FIGHTER_KIND_CAPTAIN 
            && status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_HI_THROW)
        || (fighter_kind == *FIGHTER_KIND_GANON 
            && status_kind == *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW)
        || (fighter_kind == *FIGHTER_KIND_MASTER 
            && (status_kind == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_HI_WALL_JUMP
            || status_kind == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_HI_OVERTAKE
            || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
            || status_kind == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_HI_FAIL
            || status_kind == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_HI_HIT))
        || (fighter_kind == *FIGHTER_KIND_JACK 
            && (status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_CUT
            || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
            || status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_PULL
            || status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_THROW
            || status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_PICKUP))
        || StopModule::is_damage(module_accessor)
        || StopModule::is_stop(module_accessor)
        || status_kind == *FIGHTER_STATUS_KIND_THROWN
        || status_kind == *FIGHTER_STATUS_KIND_REBIRTH
        || status_kind == *FIGHTER_STATUS_KIND_CLUNG_DIDDY
        || status_kind == *FIGHTER_STATUS_KIND_CLUNG_GANON
        || status_kind == *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN
        || status_kind == *FIGHTER_STATUS_KIND_CATCHED_GANON
        || status_kind == *FIGHTER_STATUS_KIND_MEWTWO_THROWN
        || (fighter_kind == *FIGHTER_KIND_BAYONETTA 
            && status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL)
        {
            UP_SPECIAL[ENTRY_ID] = false;
        }
    
    }
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(set_bool);
}