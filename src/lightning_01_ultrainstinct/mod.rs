use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::FighterManager;
use smash::app::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::Vector2f;
use smashline::*;
use smash_script::*;
use skyline::hooks::{getRegionAddress, Region};

//CREATED BY WUBOY, THANK YOU!!

pub static mut SECRET_SENSATION : [bool; 8] = [false; 8];
static mut CAMERA : [bool; 8] = [false; 8];
pub static mut OPPONENT_X : [f32; 8] = [0.0; 8];
pub static mut OPPONENT_Y : [f32; 8] = [0.0; 8];
pub static mut OPPONENT_BOMA : [u64; 8] = [0; 8];
static mut RYU_X : [f32; 8] = [0.0; 8];
static mut RYU_Y : [f32; 8] = [0.0; 8];
static mut SEC_SEN_TIMER : [f32; 8] = [-0.2; 8]; // I start this as -0.4 so that Ryu doesn't immediately start dodging, there's a little pause before he does
static mut OPPONENT_DIRECTION : [f32; 8] = [12.0; 8];
static mut VERT_EXTRA : [f32; 8] = [12.0; 8];
static mut SEC_SEN_STATE : [bool; 8] = [false; 8];
static mut SEC_SEN_DIREC : [i32; 8] = [0; 8];
static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20;
//pub const UI_AURA: u64 = smash::hash40("sys_cloud_limitbreak_aura");

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
    // let attacker_fighter_kind = sv_battle_object::kind(attacker_object_id);
    // let defender_fighter_kind = sv_battle_object::kind(defender_object_id);
    // let a_entry_id = WorkModule::get_int(attacker_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let d_entry_id = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if d_entry_id < 8 {
        if SEC_SEN_STATE[d_entry_id] {//If player is at 150% or up and in fall or idle animation 
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
                let oboma = sv_battle_object::module_accessor((WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
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
    
    original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}

#[fighter_frame_callback]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let fighter_kind = utility::get_kind(module_accessor);
        //let status_kind = StatusModule::status_kind(module_accessor);
        //let situation_kind = StatusModule::situation_kind(module_accessor);
       
        
        //if fighter_kind == *FIGHTER_KIND_ALL {
            if entry_id < 8 {
                
                // Setting up Secret Sensation State

                if DamageModule::damage(module_accessor,0) >= 150.0 
                && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)
                //&& (MotionModule::motion_kind(module_accessor) == smash::hash40("damage_hi_2") || MotionModule::motion_kind(module_accessor) == smash::hash40("damage_air_3"))
                && MotionModule::frame(module_accessor) <=30.0 { 
                    SEC_SEN_STATE[entry_id] = true;
                }
                else if SECRET_SENSATION[entry_id] {
                    macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
                    DamageModule::set_damage_mul(module_accessor, 0.0);
                    DamageModule::set_reaction_mul(module_accessor, 0.0);
                }
                else if SECRET_SENSATION[entry_id] == false
                && SEC_SEN_STATE[entry_id] {
                    DamageModule::set_damage_mul(module_accessor, 1.0);
                    DamageModule::set_reaction_mul(module_accessor, 1.0);
                    SEC_SEN_STATE[entry_id] = false;
                    macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
                }

                if SEC_SEN_STATE[entry_id] {
                    DamageModule::set_damage_mul(module_accessor, 0.0);
                    DamageModule::set_reaction_mul(module_accessor, 0.0);
                }

                // Reset Vars

                if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
                    SECRET_SENSATION[entry_id] = false;
                    SEC_SEN_STATE[entry_id] = false;
                }
                
                // Secret Sensation???
                
                if SECRET_SENSATION[entry_id] {
                    
                    JostleModule::set_status(module_accessor, false); // Turns off body blocking for Ryu every frame Secret Sensation is true
                    macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU); // Makes Ryu Invincible
                    DamageModule::set_damage_lock(module_accessor, true); // Makes sure Ryu doesn't take damage during the dodge
                    DamageModule::set_no_reaction_no_effect(module_accessor, true); // Makes sure Ryu doesn't take knockback.
                    if CAMERA[entry_id] == false { // Exists so all of this code will only happen once.
                        macros::CAM_ZOOM_IN_arg5(fighter, 3.0, 0.0, 1.5, 0.0, 0.0); // Sets the camera
                        macros::SLOW_OPPONENT(fighter, 100.0, 32.0); // Slows the opponent down by 100x for 32 frames
                        macros::FILL_SCREEN_MODEL_COLOR(fighter, 0, 12, 0.1, 0.1, 0.1, 0, 0.001, 0.011, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);
                        SlowModule::set_whole(module_accessor, 4, 0); // Slows ***everything*** down by a 4x. This includes the above slowdown, which probably means I should shorten the above length of time but eh
                        RYU_X[entry_id] = PostureModule::pos_x(module_accessor); // Gets Ryu's position
                        RYU_Y[entry_id] = PostureModule::pos_y(module_accessor);
                        if RYU_X[entry_id] == OPPONENT_X[entry_id] { // The reason I sometimes set Ryu's position as the Opponent's position is for this, if Ryu can't find the owner of what hit him, he will instead just dodge backwards.
                            OPPONENT_DIRECTION[entry_id] = -12.0 * PostureModule::lr(module_accessor);
                            if fighter_kind == *FIGHTER_KIND_RYU
                            || fighter_kind == *FIGHTER_KIND_KEN {
                                SEC_SEN_DIREC[entry_id] = *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B;
                            }
                            else {
                                SEC_SEN_DIREC[entry_id] = *FIGHTER_STATUS_KIND_ESCAPE;
                            }
                        }
                        else if RYU_X[entry_id] < OPPONENT_X[entry_id] { // Checks where Ryu and his Opponent are relative to each other, and sets a value so Ryu always moves *behind* the opponent
                            OPPONENT_DIRECTION[entry_id] = 12.0;
                            if PostureModule::lr(module_accessor) == -1.0 {
                                PostureModule::reverse_lr(module_accessor);
                            }
                            if fighter_kind == *FIGHTER_KIND_RYU
                            || fighter_kind == *FIGHTER_KIND_KEN {
                                SEC_SEN_DIREC[entry_id] = *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F;
                            }
                            else {
                                SEC_SEN_DIREC[entry_id] = *FIGHTER_STATUS_KIND_ESCAPE;
                            }
                        }
                        else {
                            OPPONENT_DIRECTION[entry_id] = -12.0;
                            if PostureModule::lr(module_accessor) == 1.0 {
                                PostureModule::reverse_lr(module_accessor);
                            }
                            if fighter_kind == *FIGHTER_KIND_RYU
                            || fighter_kind == *FIGHTER_KIND_KEN {
                                SEC_SEN_DIREC[entry_id] = *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F;
                            }
                            else {
                                SEC_SEN_DIREC[entry_id] = *FIGHTER_STATUS_KIND_ESCAPE;
                            }
                        }
                        if (RYU_Y[entry_id] - OPPONENT_Y[entry_id]).abs() <= 12.0
                        && StatusModule::situation_kind(OPPONENT_BOMA[entry_id] as *mut BattleObjectModuleAccessor) == *SITUATION_KIND_GROUND {
                            VERT_EXTRA[entry_id] = 0.0;
                        }
                        else {
                            StatusModule::set_situation_kind(module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
                            WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                            VERT_EXTRA[entry_id] = 12.0;
                            RYU_Y[entry_id] += 2.0;
                            PostureModule::add_pos_2d(module_accessor, &Vector2f{
                                x: 0.0,
                                y: 2.0
                            });
                        }
                        CAMERA[entry_id] = true; // Again, ensures that the above code only runs once.
                    }
                    if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR {
                        if SEC_SEN_DIREC[entry_id] == *FIGHTER_STATUS_KIND_ESCAPE {
                            SEC_SEN_DIREC[entry_id] = *FIGHTER_STATUS_KIND_ESCAPE_AIR;
                        }
                    }
                    if SEC_SEN_TIMER[entry_id] >= 0.0 { // This whole if statement is for linearly interpolating Ryu's position, instead of just teleporting him behind the opponent.
                        if (RYU_Y[entry_id] - OPPONENT_Y[entry_id]).abs() <= 12.0
                        && StatusModule::situation_kind(OPPONENT_BOMA[entry_id] as *mut BattleObjectModuleAccessor) == *SITUATION_KIND_GROUND {
                            GroundModule::correct(module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                        }
                        if StatusModule::status_kind(module_accessor) != SEC_SEN_DIREC[entry_id] { // Checks every frame if Ryu is in Focus Attack Dash state. Use another status, perhaps ESCAPE, if you're using another character
                            KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
                            StatusModule::change_status_request_from_script(module_accessor, SEC_SEN_DIREC[entry_id], true);
                        }
                        PostureModule::set_pos_2d(module_accessor, &Vector2f{ // Linear Interpolation formula: Destination * t + Starting * (1.0 - t), where 0 <= t <= 1. You can't add vectors apparently, so I did this for both X and Y.
                            x: (((OPPONENT_X[entry_id] + OPPONENT_DIRECTION[entry_id]) * SEC_SEN_TIMER[entry_id]) + RYU_X[entry_id] * (1.0 - SEC_SEN_TIMER[entry_id])),
                            y: (((OPPONENT_Y[entry_id] + VERT_EXTRA[entry_id]) * SEC_SEN_TIMER[entry_id]) + RYU_Y[entry_id] * (1.0 - SEC_SEN_TIMER[entry_id])) // There's a +12.0 so that, for moving into the air, Ryu moves slightly above the opponent. Does nothing on the ground. I may change this later.
                        });
                    }
                    SEC_SEN_TIMER[entry_id] += 0.1; // Increases the "t" in the interpolation formula by 0.1 every frame.
                    if SEC_SEN_TIMER[entry_id] > 1.0 {
                        SECRET_SENSATION[entry_id] = false;
                        CAMERA[entry_id] = false;
                        WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK); // These three lines are here to make sure Ryu doesn't just fall like a rock after movinag into the air.
                        macros::SET_SPEED_EX(fighter, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                        macros::CAM_ZOOM_OUT(fighter); // Resets the camera.
                        macros::CANCEL_FILL_SCREEN(fighter, 0, 5); // Clears out the background screen darken effect.
                        WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                        PostureModule::reverse_lr(module_accessor);
                        if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR {
                            PostureModule::reverse_lr(module_accessor); // meant to turn Ryu around if he's in the air, since he moves behind the opponent, though I just moved it here, still untested if it works
                        }
                        SlowModule::clear_whole(module_accessor); // Clears the global 4x slowdown multiplier from above
                        JostleModule::set_status(module_accessor, true); // Resets Ryu's body blocking back to normal
                        SEC_SEN_TIMER[entry_id] = -0.2; // Resets the interpolation timer.
                    }
                }
                
            }
        //}
    }
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
    smashline::install_agent_frame_callbacks!(once_per_fighter_frame);
    skyline::install_hook!(notify_log_event_collision_hit_replace);
}