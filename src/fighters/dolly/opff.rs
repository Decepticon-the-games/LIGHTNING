
use super::*;





#[fighter_frame( agent = FIGHTER_KIND_DOLLY )]
pub fn dolly_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        //let status_kind = StatusModule::status_kind(module_accessor);
        ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        ////let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        ////let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
        ////let jump_guard_dash_upspecial_pressed = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALK) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (situation_kind == *SITUATION_KIND_AIR && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0);
    //
        
        //FIXES
        //-------------------------------------------------------------------------------
        //if 
            
    

        //enable_attack_cancel(fighter); 
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_CATEGORY_MASK_ALL) {
            enable_attack_cancel(fighter); 
        } 



    }                                   
}

pub fn install() {
    smashline::install_agent_frames!(dolly_opff);
}