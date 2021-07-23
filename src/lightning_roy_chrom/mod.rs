use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use acmd;

// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        
        if fighter_kind == *FIGHTER_KIND_ROY || fighter_kind == *FIGHTER_KIND_CHROM {
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
            || status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2
            || status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3
            || status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4 {

                if status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4 {
            
                    if AttackModule:: is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)  &&  ! AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT) {
                    CancelModule::enable_cancel(module_accessor);
                } 
                }
            } 
               
            else if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3) {
                if AttackModule:: is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)  &&  ! AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT) {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
            if MotionModule::motion_kind(module_accessor) == smash::hash40("attack_11") && AttackModule::is_attack_occur(module_accessor) {
                if MotionModule::frame(module_accessor)== 3.0 && (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0) {
                    CancelModule::enable_cancel(module_accessor);
                }
            } 
        }
    }                                      
}

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
}