use super::*;
#[acmd_script( agent = "reflet", script = "game_attackairlw", category = ACMD_GAME, low_priority )]
unsafe fn game_attackairlw(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 13.0);
   
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        if macros::is_excute(fighter) {
            enable_attack_cancel(fighter); 
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 270, 100, 0, 20, 4.5, 0.0, -8.0, -0.6, Some(0.0), Some(-4.0), Some(-0.6), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 70, 100, 0, 20, 4.5, 0.0, -8.0, -0.6, Some(0.0), Some(-4.0), Some(-0.6), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 12.0, 40, 80, 0, 55, 4.2, 0.0, 6.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(fighter.module_accessor, 1, false);
        }
        frame(fighter.lua_state_agent, 18.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 8.0, 65, 80, 0, 45, 3.0, 0.0, 8.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 25.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            whiff_cancel(fighter);
        }
    }
    else{
        if macros::is_excute(fighter) {
            enable_attack_cancel(fighter); 
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 7.2, 361, 80, 0, 55, 3.7, 0.0, 6.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 6.0, 361, 80, 0, 45, 2.5, 0.0, 6.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 25.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        whiff_cancel(fighter);
        }
    } 
    frame(fighter.lua_state_agent, 27.0);
    if WorkModule::get_int(fighter.module_accessor,*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) >= 0 {

       
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            if macros::is_excute(fighter) {
                VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
            }
        }
    }

   

    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        //battle_object();
        //methodlib::L2CValue::L2CValue(void*)();
    }
    else{
        //methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
        //methodlib::L2CValue::as_pointer()const(-1, 0);
        //throwaway_sword();
    }
    frame(fighter.lua_state_agent, 48.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
    game_attackairlw);
}