use super::*;
#[acmd_script( agent = "lucario", script = "game_specialairs", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairs(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 9.0, 5.0);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 24.0);
    
    if macros::IS_GENERATABLE_ARTICLE(fighter, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG) {
        if macros::is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG, false, -1);
            whiff_cancel(fighter);
        }
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_specialairs);
}