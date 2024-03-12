use super::*;

#[fighter_frame( agent = FIGHTER_KIND_KAMUI )]

    pub fn kamui_opff(fighter : &mut L2CFighterCommon) {
        unsafe {

        }
    }


pub fn install() {
    smashline::install_agent_frames!(
        //kamui_opff, 
    );

}