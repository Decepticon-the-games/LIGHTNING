use super::*;
#[acmd_script( agent = "daisy", script = "game_specialhiopen", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhiopen(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, Hash40::new("special_hi_open"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("havel"), 2.0, 80, 15, 0, 65, 2.0, -3.1, 6.5, 0.0, Some(3.1), Some(6.5), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 18, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PEACH_PARASOL, *ATTACK_REGION_PARASOL);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
        enable_attack_cancel(fighter); 
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialhiopen);
}