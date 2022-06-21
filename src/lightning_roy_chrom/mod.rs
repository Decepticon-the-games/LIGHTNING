use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use smashline::*;




// Use this for general per-frame fighter-level hooks
#[fighter_frame( agent = FIGHTER_KIND_ROY )]
pub fn once_per_fighter_frame_roy(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
        || status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2
        || status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3
        || status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4 {

            if status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4 {
        
                            if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor){
                CancelModule::enable_cancel(fighter.module_accessor);
            }
         
            }
        } 
            
        else if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
        //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3)
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
                        if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor){
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        
        }
        if MotionModule::motion_kind(module_accessor) == smash::hash40("attack_11") && AttackModule::is_attack_occur(module_accessor) {
            if MotionModule::frame(module_accessor)== 3.0 && (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0) {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
    }                                      
}

#[fighter_frame( agent = FIGHTER_KIND_CHROM )]
pub fn once_per_fighter_frame_chrom(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
        || status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2
        || status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3
        || status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4 {

            if status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4 {
        
                            if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor){
                CancelModule::enable_cancel(fighter.module_accessor);
            }
         
            }
        } 
            
        else if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
        //&& ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3)
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
                        if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(fighter.module_accessor){
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        
        }
        if MotionModule::motion_kind(module_accessor) == smash::hash40("attack_11") && AttackModule::is_attack_occur(module_accessor) && ! SlowModule::is_slow(module_accessor) {
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
    }                                      
}

pub fn install() {
    smashline::install_agent_frames!(once_per_fighter_frame_roy, once_per_fighter_frame_chrom);
}