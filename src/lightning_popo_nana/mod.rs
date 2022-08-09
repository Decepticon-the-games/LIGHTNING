
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;


static mut STORE_TIMER : [i32; 8] = [0; 8];
static mut STATUS : [i32; 8] = [0; 8];
static mut STORE_MOVE : [bool; 8] = [false; 8];
static mut RELEASE : [bool; 8] = [false; 8];

// Use this for general per-frame fighter-level hooks
#[fighter_frame( agent = FIGHTER_KIND_POPO )]
pub fn once_per_fighter_frame_popo(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 1);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
        let jump_guard_dash_upspecial_pressed = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0);
        let frame = MotionModule::frame(module_accessor);
        
        //FIXES
        //-------------------------------------------------------------------------------
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && frame >50.0 {
                        if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor) 
 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 && frame >26.0 {
                        if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor) 
 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        
        }
        
        //else 
        if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK) 
        && ! (status_kind == *FIGHTER_STATUS_KIND_FINAL)
        && ! (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
        && ! (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3)
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
                        if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor) 
 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        
        }

    }                                      
}

#[fighter_frame( agent = FIGHTER_KIND_NANA )]
pub fn once_per_fighter_frame_nana(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 1);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
        let jump_guard_dash_upspecial_pressed = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0);
        let frame = MotionModule::frame(module_accessor);
        
        //FIXES
        //-------------------------------------------------------------------------------
        if status_kind == *FIGHTER_POPO_STATUS_KIND_SPECIAL_S_PARTNER && frame >50.0 {
                        if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor) 
 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 && frame >26.0 {
                        if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor) 
 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        
        }

        println!("something: {}", STORE_TIMER[entry_id] );

        if AttackModule::is_attack_occur(fighter.module_accessor) 
        && SlowModule::is_slow(fighter.module_accessor)
        && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
         
        { //Within HITLAG off hitting anything annd pressing special+shield 
            //STORE_TIMER[entry_id] = 1;
            STORE_MOVE[entry_id] = true;
            STATUS [entry_id] = StatusModule::status_kind(module_accessor); // Gets current status kind
            ModelModule::enable_gold_eye(module_accessor);	
        }

        if STORE_MOVE[entry_id] == true 
        //&& STORE_TIMER[entry_id] >= 1 
        {

            STORE_TIMER[entry_id] +=1; //Counts the timer up

            if STORE_TIMER[entry_id] > 300 {

                RELEASE[entry_id] = true;

                ModelModule::disable_gold_eye(module_accessor);	
                
                
            }
            if RELEASE[entry_id] {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL){
                    StatusModule::change_status_force(fighter.module_accessor, STATUS[entry_id], false);
                    //STATUS[entry_id] = -1;
                    //RELEASE[entry_id] = false;
                    STORE_MOVE[entry_id] = false;
                    STORE_TIMER[entry_id] = 0;
                }                
            }
            
        }




        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {

            STORE_TIMER[entry_id] = 0;
            STATUS[entry_id] = 0;
            STORE_MOVE[entry_id] = false;
            RELEASE[entry_id] = false;
        }
          

        //else 
        if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK) 
        && ! (status_kind == *FIGHTER_STATUS_KIND_FINAL)
        && ! (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
        && ! (status_kind == *FIGHTER_POPO_STATUS_KIND_SPECIAL_S_PARTNER)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3)
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
                        if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor) 
 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        
        }

    }                                      
}

pub fn install() {
    smashline::install_agent_frames!(once_per_fighter_frame_popo, once_per_fighter_frame_nana);
}