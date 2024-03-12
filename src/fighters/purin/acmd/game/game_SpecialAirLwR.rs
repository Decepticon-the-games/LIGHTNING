use super::*;
use crate::fighters::purin::opff::DAIR_REST_NOKILL;

#[acmd_script( agent = "purin", script = "game_specialairlwr", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairlwr(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        
        if DAIR_REST_NOKILL[entry_id] {
            macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 20.0, 88, 22, 0, 50, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BODY);
            DAIR_REST_NOKILL[entry_id] = false;
            enable_attack_cancel(fighter);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 20.0, 88, 66, 0, 100, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BODY);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        //if IS_FLAG_FIGHTER_INSTANCE_WORK_ID_FLAG_LIGHTNING[entry_id] {//In lightning whiff rest is cancelable
            whiff_cancel(fighter);
        //}
        JostleModule::set_status(fighter.module_accessor, true);
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialairlwr);
}