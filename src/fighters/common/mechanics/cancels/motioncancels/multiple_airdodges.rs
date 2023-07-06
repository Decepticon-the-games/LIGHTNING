use super::*;

#[fighter_frame_callback]
pub fn multiple_airdodges(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(fighter.module_accessor);
        let max_jumps = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        let edgde_one_wing_max_jumps = WorkModule::get_int(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_JUMP_COUNT_MAX);        
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);        

//AIRDODGE COUNTER:: Every airdodge gets counted by 1.

        if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
            if AIRDODGE_BUTTON[entry_id] == false {
                AIRDODGE_COUNT[entry_id] +=1; 
                AIRDODGE_BUTTON[entry_id] = true; // This is so the counter only runs one frame. 
            }
            CANCEL_IN_NEUTRAL [entry_id] = false; // This is so cancel in neutral only runs before cancelling, to avoid spams.                  
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

pub fn install() {
    smashline::install_agent_frame_callbacks!(multiple_airdodges);
}