use super::*;

#[fighter_frame_callback]
pub fn multiple_airdodges(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);   
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(fighter.module_accessor);
        let max_jumps = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        let edgde_one_wing_max_jumps = WorkModule::get_int(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_JUMP_COUNT_MAX);        
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);        
        let frame = MotionModule::frame(fighter.module_accessor);
        
        if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
            AIRDODGE_PLUS[entry_id] = true;
        }
        //In LIGHTNING, cancelling airdodge into airdodge is enabled.
        if IS_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING[entry_id] {
            //if [smash::hash40("escape_air"), smash::hash40("escape_air_slide")].contains(&motion_kind) 
            if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR
            {
                AIRDODGE_PLUS[entry_id] = true;
            } 
        }


//AIRDODGE COUNTER:: Every airdodge gets counted by 1.

        if [smash::hash40("escape_air"), smash::hash40("escape_air_slide")].contains(&motion_kind) { 
            if AIRDODGE_BUTTON[entry_id] == false {
                AIRDODGE_COUNT[entry_id] +=1; 
                AIRDODGE_BUTTON[entry_id] = true; // This is so the counter only runs one frame. 
            }                 
        }
        else {
            AIRDODGE_BUTTON[entry_id] = false;  
        }


//MORE THAN ONE AIRDODGE::For every jump a character has, they have that many airdodges.

        if AIRDODGE_PLUS[entry_id] {
            if (max_jumps == 2 && AIRDODGE_COUNT[entry_id] <2)
            || (max_jumps == 3 && AIRDODGE_COUNT[entry_id] <3) 
            || (max_jumps == 4 && AIRDODGE_COUNT[entry_id] <4) 
            || (max_jumps == 5 && AIRDODGE_COUNT[entry_id] <5) 
            || (max_jumps == 6 && AIRDODGE_COUNT[entry_id] <6)
            || (edgde_one_wing_max_jumps == 3 && AIRDODGE_COUNT[entry_id] <3)
            {
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                    AIRDODGE_BUTTON[entry_id] = false;//resets for the counter to count properly inbetween double airdodge
                }
            }
            AIRDODGE_PLUS[entry_id] = false;
        }
    
//Reset Airdodge count when u land
        if situation_kind == *SITUATION_KIND_GROUND { 
            AIRDODGE_COUNT[entry_id] = 0;
        }      
    }
}

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_flag )]
pub unsafe fn is_flag_replace(module_accessor: &mut BattleObjectModuleAccessor, flag: i32) -> bool {
    let ret = original!()(module_accessor, flag);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(module_accessor);
    let max_jumps = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    let edgde_one_wing_max_jumps = WorkModule::get_int(module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_JUMP_COUNT_MAX);    

    if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {

        if (max_jumps == 2 && AIRDODGE_COUNT[entry_id] <2)
        || (max_jumps == 3 && AIRDODGE_COUNT[entry_id] <3) 
        || (max_jumps == 4 && AIRDODGE_COUNT[entry_id] <4) 
        || (max_jumps == 5 && AIRDODGE_COUNT[entry_id] <5) 
        || (max_jumps == 6 && AIRDODGE_COUNT[entry_id] <6)
        || (edgde_one_wing_max_jumps == 3 && AIRDODGE_COUNT[entry_id] <3)
        {
            if flag == *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR {
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
    else {
        return ret;
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(multiple_airdodges);
    //skyline::install_hook!(is_flag_replace);
}