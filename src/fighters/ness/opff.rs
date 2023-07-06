use {
    smash::{
        lua2cpp::{L2CAgentBase,L2CFighterCommon,L2CFighterBase},
        phx::Hash40,
        hash40,
        app::{lua_bind::*, sv_animcmd::*,*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};
use crate::fighters::common::mechanics::cancels::attack_cancels::ENABLE_ATTACK_CANCEL;
use crate::fighters::common::mechanics::cancels::motioncancels::{CANCEL_IN_NEUTRAL, DISABLE_CANCEL_IN_NEUTRAL, SIDE_SPECIAL_COUNT, SIDE_SPECIAL_COUNTER};
pub static mut NO_PKFIRE : [bool; 8] = [false; 8];
#[weapon_frame( agent = WEAPON_KIND_NESS_PK_FIRE )]

    pub fn ness_pkfire_opff(weapon: &mut L2CFighterBase) {
        unsafe {
            let status_kind = StatusModule::status_kind(weapon.module_accessor);
            let oboma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
            let entry_id = WorkModule::get_int(oboma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                
            /*if NO_PKFIRE[entry_id] {

                if status_kind == WEAPON_NESS_PK_FIRE_STATUS_KIND_PILLAR {
                    //if AttackModule::is_attack_occur(weapon.module_accessor) {
                        
                        let battle_object = smash::app::sv_system::battle_object(weapon.lua_state_agent);
                        let article = &mut smash::app::Article{battle_object: *battle_object};
                        //let object_id = Article::get_battle_object_id(article) as u32;
                        ArticleModule::remove_exist_object_id(oboma,(*FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE) as u32);
                        NO_PKFIRE[entry_id] = false;
                    //}
                }
            }*/
        }
    }
        
    
            
pub fn install() {
    smashline::install_agent_frames!(
        //ness_pkfire_opff
    );

}