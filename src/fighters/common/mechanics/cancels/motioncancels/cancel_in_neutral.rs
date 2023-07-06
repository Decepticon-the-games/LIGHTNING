use super::*;

#[fighter_frame_callback]
pub fn cancel_in_neutral(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(fighter.module_accessor);

        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);

        let frame = MotionModule::frame(fighter.module_accessor);

        let max_jumps = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        let edgde_one_wing_max_jumps = WorkModule::get_int(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_JUMP_COUNT_MAX);
        let jumps_used = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        
        let grab = (motion_kind == smash::hash40("catch"))
        || (motion_kind == smash::hash40("catch_dash"))
        || (motion_kind == smash::hash40("catch_turn"));

//CANCEL FOR AS MANY JUMPS/AIRDODGES AS YOU HAVE

        if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
            AIRDODGE_PLUS[entry_id] = true;
        }
        //In LIGHTNING, cancelling airdodge into airdodge is enabled.
        if LIGHTNING[entry_id] {
            if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR 
            && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
                AIRDODGE_PLUS[entry_id] = true;
            } 
        }

        
        if ! (AttackModule::is_attack_occur(fighter.module_accessor) && ! status_kind == *FIGHTER_STATUS_KIND_THROW) {//Only on whiff, not on hit (DON'T COUNT THROWS CUZ U CANT EVEN WHIFF THOSE, plus some throws have hitboxes)

            if CANCEL_IN_NEUTRAL [entry_id] 
            {
                //AIRDODGES
                if (max_jumps == 2 && AIRDODGE_COUNT[entry_id] <2)
                || (max_jumps == 3 && AIRDODGE_COUNT[entry_id] <3) 
                || (max_jumps == 4 && AIRDODGE_COUNT[entry_id] <4) 
                || (max_jumps == 5 && AIRDODGE_COUNT[entry_id] <5) 
                || (max_jumps == 6 && AIRDODGE_COUNT[entry_id] <6)
                || (edgde_one_wing_max_jumps == 3 && AIRDODGE_COUNT[entry_id] <3)
                {
                    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 {
                        CancelModule::enable_cancel(fighter.module_accessor);
                        AIRDODGE_PLUS[entry_id] = true;//enables multiple airdadge cancels back to back
                        CANCEL_IN_NEUTRAL [entry_id] = false; // This is so cancel in neutral only runs before cancelling, to avoid spams.
                    }
                }
                //JUMPS
                if (max_jumps == 2 && jumps_used <2)
                || (max_jumps == 3 && jumps_used <3) 
                || (max_jumps == 4 && jumps_used <4) 
                || (max_jumps == 5 && jumps_used <5) 
                || (max_jumps == 6 && jumps_used <6)
                || (edgde_one_wing_max_jumps == 3 && jumps_used <3)
                {
                    if (ControlModule::is_enable_flick_jump(fighter.module_accessor) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 {
                        CancelModule::enable_cancel(fighter.module_accessor);
                        CANCEL_IN_NEUTRAL [entry_id] = false;
                    }
                }
            }

//EVERYTHING ELSE

            if CANCEL_IN_NEUTRAL [entry_id]  {
            
                if (((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0) && situation_kind == *SITUATION_KIND_GROUND && ! grab) 
                || ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 && situation_kind == *SITUATION_KIND_AIR) 
                {
                    CancelModule::enable_cancel(fighter.module_accessor);
                    CANCEL_IN_NEUTRAL [entry_id] = false;
                }
                //CANCEL_INTO_VANISH[entry_id] = true;
                ENABLE_ATTACK_CANCEL[entry_id] = false;
                            
            }
        }      

//EASIER WAVEDASH CHAINS// 
        if status_kind == *FIGHTER_STATUS_KIND_LANDING && frame >10.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }

//REMOVE LANDING LAG
        if (status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR || status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL) && frame >1.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }  
//___________________________________  
    }
}


pub fn install() {
    smashline::install_agent_frame_callbacks!(cancel_in_neutral);
    }