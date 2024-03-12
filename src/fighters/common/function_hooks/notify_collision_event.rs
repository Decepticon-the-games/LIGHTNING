use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::hash40; 
use skyline::hooks::{getRegionAddress, Region};
use smash::app::FighterManager;
use crate::fighters::common::mechanics::lightning_mechanics::ultrainstinct::{CROSS_CANCEL_SETUP, FIGHTER_STATUS_KIND_CROSS_CANCEL, OPPONENT_X, OPPONENT_Y, OPPONENT_BOMA};
use crate::fighters::common::mechanics::lightning_mechanics::vanish::{FIGHTER_STATUS_KIND_VANISH, WHO_GOT_HIT_BOMA, WHO_HIT_YOU_BOMA, PROJECTILE_BOMA, DEFENDER_VANISH, VA_OPPONENT_X, VA_OPPONENT_Y, VA_OPPONENT_BOMA};

pub static mut PROJECTILE_HIT : [bool; 8] = [false; 8];
pub static mut DIRECT_HIT : [bool; 8] = [false; 8];
pub static mut IS_HIT : [bool; 8] = [false; 8];

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
    let oboma = sv_battle_object::module_accessor((WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID)) as u32); // links weapon to whatever may ownn it  

    let attacker_fighter_kind = sv_battle_object::kind(attacker_object_id);
    //let defender_fighter_kind = sv_battle_object::kind(defender_object_id);
    let o_fighter_kind = smash::app::utility::get_kind(&mut *oboma);

    //let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let a_entry_id = WorkModule::get_int(attacker_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let d_entry_id = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let o_entry_id = WorkModule::get_int(&mut *oboma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize; //links weapon to whatever may own it

    let o_status_kind = StatusModule::status_kind(oboma); 
    
    //ULTRA INSTINCT (DEFENDER)

        if utility::get_category(&mut *defender_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {

            if (CROSS_CANCEL_SETUP[d_entry_id]
            || DEFENDER_VANISH[d_entry_id]) 
            && ! (CaptureModule::is_capture(defender_boma))
            {

                if CROSS_CANCEL_SETUP[d_entry_id] 
                && StatusModule::situation_kind(defender_boma) == *SITUATION_KIND_GROUND {

                    if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
                    || utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_ENEMY {
                        OPPONENT_BOMA[d_entry_id] = (&mut *attacker_boma as *mut BattleObjectModuleAccessor) as u64;
                        OPPONENT_X[d_entry_id] = PostureModule::pos_x(attacker_boma);
                        OPPONENT_Y[d_entry_id] = PostureModule::pos_y(attacker_boma);
                        HitModule::set_whole(defender_boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                        if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                            JostleModule::set_status(&mut *attacker_boma, false);
                        }

                    }
                    else if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_WEAPON {

                        if utility::get_category(&mut *oboma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
                            OPPONENT_X[d_entry_id] = PostureModule::pos_x(attacker_boma);
                            OPPONENT_Y[d_entry_id] = PostureModule::pos_y(attacker_boma);
                            OPPONENT_BOMA[d_entry_id] = (&mut *attacker_boma as *mut BattleObjectModuleAccessor) as u64;
                            HitModule::set_whole(defender_boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                        }
                        else {
                            OPPONENT_X[d_entry_id] = PostureModule::pos_x(oboma);
                            OPPONENT_Y[d_entry_id] = PostureModule::pos_y(oboma);
                            OPPONENT_BOMA[d_entry_id] = (&mut *oboma as *mut BattleObjectModuleAccessor) as u64;
                            HitModule::set_whole(defender_boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                            if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                                JostleModule::set_status(&mut *oboma, false);
                            }
                        }
                    }
                    else {
                        OPPONENT_X[d_entry_id] = PostureModule::pos_x(defender_boma);
                        OPPONENT_Y[d_entry_id] = PostureModule::pos_y(defender_boma);
                        OPPONENT_BOMA[d_entry_id] = (&mut *defender_boma as *mut BattleObjectModuleAccessor) as u64;
                        HitModule::set_whole(defender_boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                    }      
                    DamageModule::set_damage_lock(defender_boma, true);
                    DamageModule::set_no_reaction_no_effect(defender_boma, true); 
                    HitModule::set_hit_stop_mul(defender_boma, 0.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);                    
                    FIGHTER_STATUS_KIND_CROSS_CANCEL[d_entry_id] = true;  
                }          
                else if DEFENDER_VANISH[d_entry_id] 
                && StatusModule::situation_kind(defender_boma) == *SITUATION_KIND_AIR   
                {

                    if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
                    || utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_ENEMY {
                        VA_OPPONENT_BOMA[d_entry_id] = (&mut *attacker_boma as *mut BattleObjectModuleAccessor) as u64;
                        VA_OPPONENT_X[d_entry_id] = PostureModule::pos_x(attacker_boma);
                        VA_OPPONENT_Y[d_entry_id] = PostureModule::pos_y(attacker_boma);
                        WHO_HIT_YOU_BOMA[d_entry_id] = attacker_object_id;
                        HitModule::set_whole(defender_boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                        if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                            JostleModule::set_status(&mut *attacker_boma, false);
                        }
                    }
                    else if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_WEAPON {

                        if utility::get_category(&mut *oboma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
                            VA_OPPONENT_X[d_entry_id] = PostureModule::pos_x(attacker_boma);
                            VA_OPPONENT_Y[d_entry_id] = PostureModule::pos_y(attacker_boma);
                            VA_OPPONENT_BOMA[d_entry_id] = (&mut *attacker_boma as *mut BattleObjectModuleAccessor) as u64;
                            WHO_HIT_YOU_BOMA[d_entry_id] = attacker_object_id;
                            HitModule::set_whole(defender_boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                        }
                        else {
                            VA_OPPONENT_X[d_entry_id] = PostureModule::pos_x(oboma);
                            VA_OPPONENT_Y[d_entry_id] = PostureModule::pos_y(oboma);
                            VA_OPPONENT_BOMA[d_entry_id] = (&mut *oboma as *mut BattleObjectModuleAccessor) as u64;
                            WHO_HIT_YOU_BOMA[d_entry_id] = attacker_object_id;
                            HitModule::set_whole(defender_boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                            if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                                JostleModule::set_status(&mut *oboma, false);
                            }
                        }
                    }
                    else {
                        VA_OPPONENT_X[d_entry_id] = PostureModule::pos_x(defender_boma);
                        VA_OPPONENT_Y[d_entry_id] = PostureModule::pos_y(defender_boma);
                        VA_OPPONENT_BOMA[d_entry_id] = (&mut *defender_boma as *mut BattleObjectModuleAccessor) as u64;
                        WHO_HIT_YOU_BOMA[d_entry_id] = attacker_object_id;
                    }
                    DamageModule::set_damage_lock(defender_boma, true);
                    DamageModule::set_no_reaction_no_effect(defender_boma, true); 
                    HitModule::set_hit_stop_mul(defender_boma, 0.0, smash::app::HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0); 
                    FIGHTER_STATUS_KIND_VANISH[d_entry_id] = true;  
                }  
            } 
        }

    //VANISH(ATTACKER)


            //IF THE ATTACKER IS A FIGHTER AND THE DEFENDER IS A FIGHTER, GET THE DEFENNDER'S POSITION

                if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER // if the attacker is a fighter
                {               
                    if utility::get_category(&mut *defender_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER 
                    || utility::get_category(&mut *defender_boma) == *BATTLE_OBJECT_CATEGORY_ENEMY { // if the defender is a fighter/enemy
                        
                        DIRECT_HIT[a_entry_id] = true;
                        WHO_GOT_HIT_BOMA[a_entry_id] = defender_object_id; //Store the id of the person who got hit up until vanish is pressed
                    } 
                }    
            
        
            
            //IF THE ATTACKER IS A WEAPON 
                if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_WEAPON {//if the attacker is a weaponn (projectile) 

                    // Check to see if the owner of what you hit is a Fighter or not. If yes...

                    if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER { // If the object that was hit is owned by a fighter, stores that fighter's position
                    
                        PROJECTILE_BOMA[o_entry_id] = attacker_object_id; //Store the id of the person the attacking projectile belongs to until vanish is pressed
                        WHO_GOT_HIT_BOMA[o_entry_id] = defender_object_id; //Store the id of the person who got hit up until vanish is pressed
                        PROJECTILE_HIT[o_entry_id] = true;
                    }    

                    //If a weapon...
                
                    else if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_WEAPON { // If the object that was hit is a fighter, stores the opponent's position

                        PROJECTILE_BOMA[o_entry_id] = attacker_object_id; //Store the id of the person the attacking projectile belongs to until vanish is pressed
                        WHO_GOT_HIT_BOMA[o_entry_id] = defender_object_id; //Store the id of the person who got hit up until vanish is pressed
                        PROJECTILE_HIT[o_entry_id] = true;
                    }
                }                 

    original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
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
}