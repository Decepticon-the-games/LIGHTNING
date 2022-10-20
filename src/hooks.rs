
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::hash40; 
use skyline::hooks::{getRegionAddress, Region};
use smash::app::FighterManager;
use crate::lightning_01_ultrainstinct::{SEC_SEN_STATE, SECRET_SENSATION, OPPONENT_X, OPPONENT_Y, OPPONENT_BOMA, CROSS_CANCEL_BUTTON};
use crate::lightning_01_vanish::{ACTIVATE_VANISH, VANISH, VANISH_READY, WHO_GOT_HIT_BOMA, VANISH_BUTTON};
use crate::lightning_01_upbtransitions::DISABLE_UP_SPECIAL;
use crate::lightning_01_lightning_fsmeter::{DISABLE_FINAL, FINAL_SMASH_BUTTON};
use crate::lightning_01_crimson_cancel::{CRIMSON_CANCEL_BUTTON, CRIMSON_CANCEL_TIMER};
use crate::lightning_01_lightning_mode::{LIGHTNING_BUTTON};

pub static mut PROJECTILE_HIT : [bool; 8] = [false; 8];
pub static mut DIRECT_HIT : [bool; 8] = [false; 8];

static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20;
static NOTIFY_LOG_EVENT_COLLISION_HIT_SEARCH_CODE: &[u8] = &[
    0xff, 0x03, 0x03, 0xd1,
    0xe8, 0x2b, 0x00, 0xfd,
    0xfc, 0x6f, 0x06, 0xa9,
    0xfa, 0x67, 0x07, 0xa9,
    0xf8, 0x5f, 0x08, 0xa9,
    0xf6, 0x57, 0x09, 0xa9,
    0xf4, 0x4f, 0x0a, 0xa9,
    0xfd, 0x7b, 0x0b, 0xa9,
    0xfd, 0xc3, 0x02, 0x91,
    0xfb, 0x03, 0x00, 0xaa
];
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}



#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn notify_log_event_collision_hit_replace(
fighter_manager: &mut FighterManager,
attacker_object_id: u32,
defender_object_id: u32, 
move_type: f32,
arg5: i32,
move_type_again: bool) -> u64 {
    
    let attacker_boma = sv_battle_object::module_accessor(attacker_object_id);
    let defender_boma = sv_battle_object::module_accessor(defender_object_id);

    let attacker_fighter_kind = sv_battle_object::kind(attacker_object_id);
    //let defender_fighter_kind = sv_battle_object::kind(defender_object_id);

    //let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let a_entry_id = WorkModule::get_int(attacker_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let d_entry_id = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    let o_fighter_kind = smash::app::utility::get_kind(&mut *oboma);
    let o_status_kind = StatusModule::status_kind(oboma); 
    //ULTRA INSTINCT (DEFENDER)

        if utility::get_category(&mut *defender_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {

            if SEC_SEN_STATE[d_entry_id] {
                if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
                || utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_ENEMY {
                    OPPONENT_BOMA[d_entry_id] = (&mut *attacker_boma as *mut BattleObjectModuleAccessor) as u64;
                    OPPONENT_X[d_entry_id] = PostureModule::pos_x(attacker_boma);
                    OPPONENT_Y[d_entry_id] = PostureModule::pos_y(attacker_boma);
                    if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                        JostleModule::set_status(&mut *attacker_boma, false);
                    }
                }
                else if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_WEAPON {
                    let oboma = sv_battle_object::module_accessor((WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID)) as u32); // links weapon to whatever may ownn it  
                    let o_entry_id = WorkModule::get_int(&mut *oboma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; //links weapon to whatever may own it

                    if utility::get_category(&mut *oboma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
                        OPPONENT_X[d_entry_id] = PostureModule::pos_x(defender_boma);
                        OPPONENT_Y[d_entry_id] = PostureModule::pos_y(defender_boma);
                        OPPONENT_BOMA[d_entry_id] = (&mut *defender_boma as *mut BattleObjectModuleAccessor) as u64;
                    }
                    else {
                        OPPONENT_X[d_entry_id] = PostureModule::pos_x(oboma);
                        OPPONENT_Y[d_entry_id] = PostureModule::pos_y(oboma);
                        OPPONENT_BOMA[d_entry_id] = (&mut *oboma as *mut BattleObjectModuleAccessor) as u64;
                        if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                            JostleModule::set_status(&mut *oboma, false);
                        }
                    }
                }
                else {
                    OPPONENT_X[d_entry_id] = PostureModule::pos_x(defender_boma);
                    OPPONENT_Y[d_entry_id] = PostureModule::pos_y(defender_boma);
                    OPPONENT_BOMA[d_entry_id] = (&mut *defender_boma as *mut BattleObjectModuleAccessor) as u64;
                }
                SECRET_SENSATION[d_entry_id] = true;
            }
        }

    //VANISH (ATTACKER)

        

        
        //if ! (o_fighter_kind == *FIGHTER_KIND_POPO || o_fighter_kind == *FIGHTER_KIND_NANA)
        //&& ! (o_status_kind == *FIGHTER_STATUS_KIND_FINAL)
        //{   

            //if ACTIVATE_VANISH[o_entry_id] {
            //IF THE ATTACKER IS A FIGHTER AND THE DEFENDER IS A FIGHTER, GET THE DEFENNDER'S POSITION

                if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER // if the attacker is a fighter
                //&& ! StatusModule::status_kind(attacker_boma) == *FIGHTER_STATUS_KIND_CATCH_ATTACK 
                {               
                    if utility::get_category(&mut *defender_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER 
                    || utility::get_category(&mut *defender_boma) == *BATTLE_OBJECT_CATEGORY_ENEMY { // if the defender is a fighter/enemy
                        
                        DIRECT_HIT[a_entry_id] = true;
                        WHO_GOT_HIT_BOMA[a_entry_id] = defender_object_id; //Store the id of the person who got hit up until vanish is pressed
                        //VANISH_READY[a_entry_id] = true; 
                    } 
                }    
            
        
            
            //IF THE ATTACKER IS A WEAPON 
                if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_WEAPON {//if the attacker is a weaponn (projectile) 
                    let oboma = sv_battle_object::module_accessor((WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID)) as u32); // links weapon to whatever may ownn it  
                    let o_entry_id = WorkModule::get_int(&mut *oboma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; //links weapon to whatever may own it

                    // Check to see if the owner of what you hit is a Fighter or not. If yes...

                    if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER { // If the object that was hit is owned by a fighter, stores that fighter's position
                    
                        PROJECTILE_HIT[o_entry_id] = true; 
                        WHO_GOT_HIT_BOMA[o_entry_id] = defender_object_id; //Store the id of the person who got hit up until vanish is pressed
                        //VANISH_READY[o_entry_id] = true; 
                    }    
                

                    //If a weapon...
                
                    else if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_WEAPON { // If the object that was hit is a fighter, stores the opponent's position

                        PROJECTILE_HIT[o_entry_id] = true; 
                        WHO_GOT_HIT_BOMA[o_entry_id] = defender_object_id; //Store the id of the person who got hit up until vanish is pressed
                        //VANISH_READY[o_entry_id] = true; 

                    }
                    
                    if o_fighter_kind == *FIGHTER_KIND_NANA { // if the projectile belongs to nana
                        PROJECTILE_HIT[o_entry_id] = true;
                    }
                }                 
            //}

                   
        //}
    //ICE CLIMBERS DESYNC RECALL

    



    original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}


#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term )]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let ret = original!()(module_accessor,term);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let special_mechanics_button = (CRIMSON_CANCEL_BUTTON[entry_id]
    || CROSS_CANCEL_BUTTON[entry_id]
    || VANISH_BUTTON[entry_id]
    || LIGHTNING_BUTTON[entry_id]);

    
    if SECRET_SENSATION[entry_id] {
        return false;
    }
    if VANISH[entry_id] {
        return false;
    }
    //if special_mechanics_button 
    //{
    //    return false;
    //}


    if DISABLE_UP_SPECIAL[entry_id] {
        if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI {
            return false;
        }
        else {
            return ret;
        }
    }
    if DISABLE_FINAL[entry_id] {
        if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL {
            return false;
        }
        else {
            return ret;
        }
    }
    if CRIMSON_CANCEL_TIMER[entry_id] >= 1 {
        if (term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK 
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
        ) {
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

#[skyline::hook(offset = 0x4E53C0)]
pub unsafe fn get_param_int_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let _fighter_kind = utility::get_kind(boma);
    if param_hash == hash40("hit_stop_delay_flick") {
        if FighterUtil::is_hp_mode(boma) {
            return 0x4;
        }
    }
    //return ret;
    //if param_hash == hash40("precede") { //No buffer during neutral at all until comboing (attacks hit)
    //    if AttackModule::is_attack_occur(boma) {
    //        return 0x10;
    //    }
    //    else{
    //        return 0x1;
    //    }
    //}
    return ret;
}

#[skyline::hook(offset = 0x4E5380)]
pub unsafe fn get_param_float_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let _fighter_kind = utility::get_kind(boma);
    if param_hash == hash40("hit_stop_delay_stick") {
        if FighterUtil::is_hp_mode(boma) {
            return 0.7;
        }
    }

    return ret;
    
}



pub fn install() {
    unsafe{
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, NOTIFY_LOG_EVENT_COLLISION_HIT_SEARCH_CODE) {
            NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
    }
    skyline::install_hook!(notify_log_event_collision_hit_replace);
    skyline::install_hook!(is_enable_transition_term_replace);
    //skyline::install_hook!(get_param_float_replace);
    skyline::install_hook!(get_param_int_replace);
}