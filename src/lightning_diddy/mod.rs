
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon};
use smash::lib::lua_const::*;


// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
        let jump_guard_dash_upspecial_pressed = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0;
        //
        
        
        if fighter_kind == *FIGHTER_KIND_DIDDY {

            //FIXES
            //-------------------------------------------------------------------------------
            //Fix up smash
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
                if AttackModule:: is_attack_occur(module_accessor) {
                    if  jump_guard_dash_upspecial_pressed {
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }
                
                
        
            //else 
            else if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100) {
                if AttackModule:: is_attack_occur(module_accessor) {
                    if MotionModule::frame(module_accessor) >1.0 { 
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }

            //ENHANCES
            //--------------------------------------------------------------------------------


            //MOTION CANCELS
            //--------------------------------------------------------------------------------

        }
        
    }                                      
}

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
}