use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::Vector2f;
use smashline::*;
use smash_script::*;
use smash::phx::Hash40;


pub static mut VANISH : [bool; 8] = [false; 8];
pub static mut VANISH_READY : [bool; 8] = [false; 8];
static mut CAMERA : [bool; 8] = [false; 8];
pub static mut VA_OPPONENT_X : [f32; 8] = [0.0; 8];
pub static mut VA_OPPONENT_Y : [f32; 8] = [0.0; 8];
pub static mut VA_OPPONENT_BOMA  : [u64; 8] = [0; 8];
static mut YOU_X : [f32; 8] = [0.0; 8];
static mut YOU_Y : [f32; 8] = [0.0; 8];
static mut VANISH_TIMER : [f32; 8] = [0.0; 8];
pub static mut ACTIVATE_VANISH : [bool; 8] = [true; 8];
static mut VERT_EXTRA : [f32; 8] = [12.0; 8];
static mut VA_OPPONENT_DIRECTION : [f32; 8] = [12.0; 8];

// VANISH

    #[fighter_frame_callback]
    pub fn vanish(fighter : &mut L2CFighterCommon) {
        unsafe {
            //let fighter.module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let status_kind = StatusModule::status_kind(fighter.module_accessor);
            let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
            if entry_id < 8 {

                // Reset Vars

                if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
                    //VANISH_READY[entry_id] = false;
                    //ACTIVATE_VANISH[entry_id] = false;
                    //VANISH[entry_id] = false;
                    
                }

                if VANISH_READY[entry_id]{
                    if ! SlowModule::is_slow(fighter.module_accessor)
                    && ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0)
                    {
                        if AttackModule::is_attack_occur(fighter.module_accessor) { // direct attacks
                            VANISH[entry_id] = true; 
                        }
                        else { // projectile attacks
                            
                            VANISH[entry_id] = true;  
                             
                        }
                        
                        VANISH_READY[entry_id] = false;

                    }
                }
                // Secret Sensation???

                
                if VANISH[entry_id] {
                    PostureModule::reverse_lr(fighter.module_accessor); 
                    //EffectModule::req_emit(fighter.module_accessor, Hash40::new("sys_aura_light"), 0);
                    //macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.851, 1.0);
                    VisibilityModule::set_whole(fighter.module_accessor, false);
                    JostleModule::set_status(fighter.module_accessor, false); // Turns off body blocking for Ryu every frame Secret Sensation is true
                    macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU); // Makes Ryu invincible.
                    MotionModule::set_rate(fighter.module_accessor, 0.001);
                    macros::SLOW_OPPONENT(fighter, 100.0, 10.0);

                    if CAMERA[entry_id] == false { // Exists so all of this code will only happen once.
                        
                        
                        YOU_X[entry_id] = PostureModule::pos_x(fighter.module_accessor); // Gets Ryu's position
                        YOU_Y[entry_id] = PostureModule::pos_y(fighter.module_accessor);
                        if YOU_X[entry_id] == VA_OPPONENT_X[entry_id] { // The reason I sometimes set Ryu's position as the Opponent's position is for this, if Ryu can't find the owner of what hit him, he will instead just dodge backwards.
                            VA_OPPONENT_DIRECTION[entry_id] = -12.0 * PostureModule::lr(fighter.module_accessor);
                        }
                        else if YOU_X[entry_id] < VA_OPPONENT_X[entry_id] { // Checks where Ryu and his Opponent are relative to each other, and sets a value so Ryu always moves *behind* the opponent
                            VA_OPPONENT_DIRECTION[entry_id] = 12.0;
                            if PostureModule::lr(fighter.module_accessor) == -1.0 {
                                PostureModule::reverse_lr(fighter.module_accessor);
                            }
                        }
                        else {
                            VA_OPPONENT_DIRECTION[entry_id] = -12.0;
                            if PostureModule::lr(fighter.module_accessor) == 1.0 {
                                PostureModule::reverse_lr(fighter.module_accessor);
                            }
                        }
                        if (YOU_Y[entry_id] - VA_OPPONENT_Y [entry_id]).abs() <= 12.0 // Checks to see if Ryu and his opponent are "close enough" in Y value to do the grounded slide instead
                        && StatusModule::situation_kind(VA_OPPONENT_BOMA [entry_id] as *mut smash::app::BattleObjectModuleAccessor) == *SITUATION_KIND_GROUND {
                            VERT_EXTRA[entry_id] = 0.0; // If Ryu and his opponent are "close enough" in vertical height, don't add any extra vertical distance.
                        }
                        else {
                            StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), false); // Sets Ryu to airborne
                            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE); // Turns of Gravity for Ryu
                            VERT_EXTRA[entry_id] = 12.0; // Makes Ryu dodge above the opponent, if they're far enough apart in Y values
                            YOU_Y[entry_id] += 2.0; // Sets Ryu's position to be slightly higher than on the ground, so he can do his aerial Focus Attack Dash animation.
                            PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                                x: 0.0,
                                y: 2.0
                            });
                        }
                        CAMERA[entry_id] = true; // Again, ensures that the above code only runs once.
                    }
                    
                    if VANISH_TIMER[entry_id] >= 0.0 { // This whole if statement is for linearly interpolating Ryu's position, instead of just teleporting him behind the opponent.
                        if (YOU_Y[entry_id] - VA_OPPONENT_Y [entry_id]).abs() > 12.0 { // If Ryu's Y and the Opponent's Y are far enough apart, do the following:
                            StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), false); // Set Ryu to be airborne
                        }
                        PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f{ // Linear Interpolation formula: Destination * t + Starting * (1.0 - t), where 0 <= t <= 1. You can't add vectors apparently, so I did this for both X and Y.
                            x: (((VA_OPPONENT_X[entry_id] + VA_OPPONENT_DIRECTION[entry_id]) * VANISH_TIMER[entry_id]) + YOU_X[entry_id] * (1.0 - VANISH_TIMER[entry_id])),
                            y: (((VA_OPPONENT_Y [entry_id] + VERT_EXTRA[entry_id]) * VANISH_TIMER[entry_id]) + YOU_Y[entry_id] * (1.0 - VANISH_TIMER[entry_id])) // There's a +12.0 so that, for moving into the air, Ryu moves slightly above the opponent. Does nothing on the ground. I may change this later.
                        });
                    }
                    VANISH_TIMER[entry_id] += 0.1; // Increases the "t" in the interpolation formula by 0.1 every frame.
                    if VANISH_TIMER[entry_id] > 1.0 {
                        VANISH[entry_id] = false;
                        CAMERA[entry_id] = false;
                        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE); // Gives Ryu back his gravity
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK); // These three lines are here to make sure Ryu doesn't just fall like a rock after moving into the air.
                        macros::SET_SPEED_EX(fighter, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                        VisibilityModule::set_whole(fighter.module_accessor, true);
                        MotionModule::set_rate(fighter.module_accessor, 1.0);
                        macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
                        JostleModule::set_status(fighter.module_accessor, true); // Resets Ryu's body blocking back to normal
                        
                        VANISH_TIMER[entry_id] = 0.0; // Resets the interpolation timer.
                    }
                }

                else {
                    //ACTIVATE_VANISH[entry_id] = true;
                    VisibilityModule::set_whole(fighter.module_accessor, true);
                    macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
                }

            }
        }
    }

pub fn install() {

   
    smashline::install_agent_frame_callbacks!(vanish);
}