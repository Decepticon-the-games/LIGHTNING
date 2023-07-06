use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*,*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

use crate::fighters::common::mechanics::lightning_mechanics::vanish::{VANISH_READY, CANCEL_INTO_VANISH};
use crate::fighters::common::function_hooks::cross_cancel_vanish::PROJECTILE_HIT;

pub static mut ATTACK_CANCEL : [bool; 8] = [false; 8];
pub static mut ENABLE_ATTACK_CANCEL : [bool; 8] = [false; 8];
pub static mut ATTACK_CANCEL_COUNTER : [bool; 8] = [false; 8];
pub static mut ATTACK_CANCEL_COUNT : [i32; 8] = [0; 8];
pub static mut ENABLE_MULTIHIT_CANCEL : [bool; 8] = [false; 8];
pub static mut MOVEMENT_CANCEL : [bool; 8] = [true; 8];

#[fighter_frame_callback]
pub fn attack_cancels(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);     
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        let movement_cancel = (ControlModule::is_enable_flick_jump(fighter.module_accessor) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0) 
        || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 
        || (((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0) && situation_kind == *SITUATION_KIND_GROUND) 
        || (((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0) && situation_kind == *SITUATION_KIND_AIR);

        let idles = (status_kind == *FIGHTER_STATUS_KIND_WAIT
        || status_kind == *FIGHTER_STATUS_KIND_FALL
        || status_kind == *FIGHTER_STATUS_KIND_FALL_AERIAL);

        let walks_runs_jumps_falls = (status_kind == *FIGHTER_STATUS_KIND_WALK
        || status_kind == *FIGHTER_STATUS_KIND_DASH
        || status_kind == *FIGHTER_STATUS_KIND_TURN_DASH
        || status_kind == *FIGHTER_STATUS_KIND_JUMP
        || status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL);


        //println!("occur: {}", AttackModule::is_attack_occur(fighter.module_accessor));
        //println!("is: {}", AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL));
        //println!("has: {}", AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL));
        //println!("en-multi: {}", ENABLE_MULTIHIT_CANCEL[entry_id]);
        //println!("en-attack: {}", ENABLE_ATTACK_CANCEL[entry_id]);

        



        //NON CANCELLABLE MOVES
        /*if (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK) 
        || (status_kind == *FIGHTER_STATUS_KIND_FINAL)
        || (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && ! fighter_kind == *FIGHTER_KIND_DONKEY)
        || (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        || (status_kind == *FIGHTER_STATUS_KIND_THROW) {
            
            ENABLE_ATTACK_CANCEL[entry_id] = false;
        }*/ 

/* TO DOS
-------------

    * Jabs can only cancel twice in a row before having to use a movement option



    * Create a system that stores the previous three moves and prevents from cancelling into them

*/

//ENABLE ATTACK CANCELS// this instance prevents cancelling more than a certain amount without first moving
        
        if ENABLE_ATTACK_CANCEL[entry_id]
        {
            if AttackModule::is_attack_occur(fighter.module_accessor) {
                if ATTACK_CANCEL_COUNTER[entry_id] == false {
                    ATTACK_CANCEL_COUNT[entry_id] +=1;
                    ATTACK_CANCEL_COUNTER[entry_id] = true;
                    //ENABLE_ATTACK_CANCEL[entry_id] = false; 
                }
                //the amount of times u can cancel in a row
                if ATTACK_CANCEL_COUNT[entry_id] > 2 {
                    ATTACK_CANCEL_COUNT[entry_id] = 3;
                    ATTACK_CANCEL[entry_id] = false;       
                } 
                else {
                    ATTACK_CANCEL[entry_id] = true;
                }            
            }
            else {
                ATTACK_CANCEL_COUNTER[entry_id] = false; 
            }
            //moovement options, always reset attack cancel count
            if movement_cancel {
                ATTACK_CANCEL_COUNT[entry_id] = 0; 
            }  
            //ENABLE_ATTACK_CANCEL[entry_id] = false;
        } 
        


//ATTACK CANCELS
        if ATTACK_CANCEL[entry_id] 
        && ! (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STATUS) || status_kind == *FIGHTER_STATUS_KIND_FINAL) {// This line is not needed after all characters get the acmd rendition
            
            //If an attack occurs, cancel out of it (after hitlag). Includes vanish
            if (AttackModule::is_attack_occur(fighter.module_accessor) || AttackModule::is_attack_occur(oboma)) && ! SlowModule::is_slow(fighter.module_accessor) { 
                CancelModule::enable_cancel(fighter.module_accessor);
                ENABLE_ATTACK_CANCEL[entry_id] = false;
                ATTACK_CANCEL[entry_id] = false; 
                CANCEL_INTO_VANISH[entry_id] = true;
            }        
        }
        else  {
            ATTACK_CANCEL[entry_id] = false;
        } 

//RESETS/FALSES
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
            ATTACK_CANCEL_COUNT[entry_id] = 0;
            ENABLE_ATTACK_CANCEL[entry_id] = false;

        } 
        if idles || walks_runs_jumps_falls{
            ENABLE_ATTACK_CANCEL[entry_id] = false;
            ENABLE_MULTIHIT_CANCEL[entry_id] = false;
        }

    }
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(attack_cancels);
}