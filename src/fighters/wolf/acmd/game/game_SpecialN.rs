use super::*;
#[acmd_script( agent = "wolf", script = "game_specialn", category = ACMD_GAME, low_priority )]
unsafe fn game_specialn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, false, -1);
    }
    if macros::IS_EXIST_ARTICLE(fighter, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER) {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, Hash40::new("open"), false, -1.0);
        }
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        ENABLE_ATTACK_CANCEL[entry_id] = true; 
macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 7.0, 60, 37, 0, 80, 2.5, 0.0, 0.0, -3.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.0, 60, 37, 0, 80, 2.5, 0.0, 0.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 7.0, 60, 37, 0, 80, 2.5, 0.0, 0.0, 5.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        CANCEL_IN_NEUTRAL[entry_id] = true;
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 38.0);
    if macros::IS_EXIST_ARTICLE(fighter, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER) {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, Hash40::new("close"), false, -1.0);
        }
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialn);
}