use super::*;
#[acmd_script( agent = "pfushigisou", script = "game_specialn", category = ACMD_GAME, low_priority )]
unsafe fn game_specialn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    wait_loop_clear(fighter.lua_state_agent);
    frame(fighter.lua_state_agent, 1.0);
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_SEED, false, -1);
    CANCEL_IN_NEUTRAL[entry_id] = true;
    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 100, 100, 110, 0, 4.0, 0.0, 4.5, 3.5, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 100, 100, 110, 0, 4.0, 0.0, 4.5, -3.5, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialn );
}
