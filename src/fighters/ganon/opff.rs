use super::*;
pub static mut UPTILT : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_GANON )]

    pub fn ganon_opff(fighter : &mut L2CFighterCommon) {
        unsafe {
            let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
            let status_kind = StatusModule::status_kind(module_accessor);
            //let motion_kind = MotionModule::motion_kind(fighter.module_accessor);       
            let frame = MotionModule::frame(fighter.module_accessor);
            ////let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
            let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            //let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
//Cancel up tilt to the attack once a hitbox connects
            if UPTILT[entry_id] {
                if AttackModule::is_attack_occur(fighter.module_accessor) && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
                    MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 53.0, false, false, false);
                }
            }
                
//Taunt/Attack cancel side special
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
                if frame>= 18.0 {
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
                    || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)
                    || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
                    || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
                    || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                        CancelModule::enable_cancel(fighter.module_accessor);
                    }
                }
            }

        }
    }

pub fn install() {
    smashline::install_agent_frames!(ganon_opff);

}