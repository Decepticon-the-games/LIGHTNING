use super::*;

#[fighter_frame_callback]
pub fn shield_opff(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        //REMOVE INVINCIBILITY ON SHIELD BREAK 
                
        if status_kind == *FIGHTER_STATUS_KIND_SHIELD_BREAK_FALL {
            macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
        }
    }
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(shield_opff);

} 