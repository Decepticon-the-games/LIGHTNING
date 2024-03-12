use super::*;
use crate::fighters::common::mechanics::cancels::motioncancels::{CANCEL_IN_NEUTRAL, PROJECTILE_COUNT, PROJECTILE_COUNTER};
pub static mut NO_PKFIRE : [bool; 8] = [false; 8];
#[weapon_frame( agent = WEAPON_KIND_NESS_PK_FIRE )]

    pub fn ness_pkfire_opff(weapon: &mut L2CFighterBase) {
        unsafe {
            let status_kind = StatusModule::status_kind(weapon.module_accessor);
            let oboma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
            let entry_id = WorkModule::get_int(oboma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                
            if NO_PKFIRE[entry_id] == false {

                if status_kind == WEAPON_NESS_PK_FIRE_STATUS_KIND_PILLAR {
                    //if AttackModule::is_attack_occur(weapon.module_accessor) {
                        NO_PKFIRE[entry_id] = true;
                    //}
                }
                else {
                    NO_PKFIRE[entry_id] = false; 
                }
            }
        }
    }
        
    
            
pub fn install() {
    smashline::install_agent_frames!(
        //ness_pkfire_opff
    );

}