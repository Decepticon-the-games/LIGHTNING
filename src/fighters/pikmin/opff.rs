use super::*;
pub static mut PIKMIN_PIKMIN_ATTACK_CANCEL : [bool; 8] = [false; 8];
pub static mut PIKMIN_ATTACK_CANCEL : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_PIKMIN )]

    pub fn pikmin_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            let status_kind = StatusModule::status_kind(module_accessor);
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
            println!("olimar {}", PIKMIN_ATTACK_CANCEL[entry_id]);

            if PIKMIN_ATTACK_CANCEL[entry_id]  {
                CancelModule::enable_cancel(fighter.module_accessor);
                PIKMIN_ATTACK_CANCEL[entry_id] = false;
            }

//New subtititle for any other code, if not applicable just delete the lines
            
        }
    }

#[weapon_frame( agent = WEAPON_KIND_PIKMIN_PIKMIN )]

    pub fn pikmin_pikmin_opff(fighter : &mut L2CFighterBase) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            let status_kind = StatusModule::status_kind(module_accessor);
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
            println!("pikmin {}", PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id]);

        if PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id]  {
            if (AttackModule::is_attack_occur(fighter.module_accessor) && SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) == 0) {
                PIKMIN_ATTACK_CANCEL[entry_id] = true;
                PIKMIN_PIKMIN_ATTACK_CANCEL[entry_id] = false;
            }  
        }

//New subtititle for any other code, if not applicable just delete the lines

        }
    }

pub fn install() {
    smashline::install_agent_frames!(pikmin_opff, pikmin_pikmin_opff);

}