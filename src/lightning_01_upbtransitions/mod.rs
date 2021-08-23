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
        let motion_kind = MotionModule::motion_kind(module_accessor);      
        ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        //Set up bool to make all types of aerial up bs to only be used once no matter what
        //if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR
        if motion_kind == smash::hash40("special_air_hi")

        || motion_kind == smash::hash40("special_air_hi_1")
        || motion_kind == smash::hash40("special_air_hi_2")

        || motion_kind == smash::hash40("special_air_hi_break")
        || motion_kind == smash::hash40("special_air_hi_ceil")
        || motion_kind == smash::hash40("special_air_hi_charge")
        || motion_kind == smash::hash40("special_air_hi_charge_b")

        || motion_kind == smash::hash40("special_air_hi_charge_f")
        || motion_kind == smash::hash40("special_air_hi_charge_hi")
        || motion_kind == smash::hash40("special_air_hi_charge_lw")
        || motion_kind == smash::hash40("special_air_hi_charge_start_b")
        || motion_kind == smash::hash40("special_air_hi_charge_start_f")
        || motion_kind == smash::hash40("special_air_hi_charge_start_hi")
        || motion_kind == smash::hash40("special_air_hi_charge_start_lw")
        || motion_kind == smash::hash40("special_air_hi_command")
        || motion_kind == smash::hash40("special_air_hi_cut")
        || motion_kind == smash::hash40("special_air_hi_damage")
        || motion_kind == smash::hash40("special_air_hi_detach")
        || motion_kind == smash::hash40("special_air_hi_empty")


        || motion_kind == smash::hash40("special_air_hi_fall")
        || motion_kind == smash::hash40("special_air_hi_fall_2")
        || motion_kind == smash::hash40("special_air_hi_flap1")
        || motion_kind == smash::hash40("special_air_hi_flap2")
        || motion_kind == smash::hash40("special_air_hi_get")
        || motion_kind == smash::hash40("special_air_hi_hang")
        || motion_kind == smash::hash40("special_air_hi_hit")
        || motion_kind == smash::hash40("special_air_hi_hit_pose")
        || motion_kind == smash::hash40("special_air_hi_hold")
        || motion_kind == smash::hash40("special_air_hi_hover")
        || motion_kind == smash::hash40("special_air_hi_j_damage")
        || motion_kind == smash::hash40("special_air_hi_jump")
        || motion_kind == smash::hash40("special_air_hi_l")
        || motion_kind == smash::hash40("special_air_hi_landing1")
        || motion_kind == smash::hash40("special_air_hi_landing2")
        || motion_kind == smash::hash40("special_air_hi_lb")
        || motion_kind == smash::hash40("special_air_hi_loop")
        || motion_kind == smash::hash40("special_air_hi_overtake")
        || motion_kind == smash::hash40("special_air_hi_pull")
        || motion_kind == smash::hash40("special_air_hi_pull_2")
        || motion_kind == smash::hash40("special_air_hi_r")
        || motion_kind == smash::hash40("special_air_hi_r")
        || motion_kind == smash::hash40("special_air_hi_reflect")
        || motion_kind == smash::hash40("special_air_hi_start")
        || motion_kind == smash::hash40("special_air_hi_start_2")
        || motion_kind == smash::hash40("special_air_hi_start_l")
        || motion_kind == smash::hash40("special_air_hi_start_nana")
        || motion_kind == smash::hash40("special_air_hi_start_r")
        || motion_kind == smash::hash40("special_air_hi_throw")
        || motion_kind == smash::hash40("special_air_hi_throw2")
        || motion_kind == smash::hash40("special_air_hi_turn")
        || motion_kind == smash::hash40("special_air_hi_turn_l")
        || motion_kind == smash::hash40("special_air_hi_turn_r")
        || motion_kind == smash::hash40("special_air_hi_wait1")
        || motion_kind == smash::hash40("special_air_hi_wait2")
        || motion_kind == smash::hash40("special_air_hi_wall")
        || motion_kind == smash::hash40("special_air_hi_wall_b")

        || motion_kind == smash::hash40("special_air_hi_wall_jump")
        || motion_kind == smash::hash40("special_air_hi1")
        || motion_kind == smash::hash40("special_air_hi1_2")
        || motion_kind == smash::hash40("special_air_hi1_3")
        || motion_kind == smash::hash40("special_air_hi1_loop")
        || motion_kind == smash::hash40("special_air_hi1_start")
        || motion_kind == smash::hash40("special_air_hi2")
        || motion_kind == smash::hash40("special_air_hi2")
        || motion_kind == smash::hash40("special_air_hi2_squat")

        || motion_kind == smash::hash40("special_air_hi3_start")
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