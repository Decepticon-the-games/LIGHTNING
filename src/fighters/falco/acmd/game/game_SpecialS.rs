use super::*;
#[acmd_script( agent = "falco", script = "game_specials", category = ACMD_GAME, low_priority )]
unsafe fn game_specials(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
let inputs = ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 ) 
        || ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 )
        || ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 )
        || ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 )
        || ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 );

    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ILLUSION, false, -1);
        macros::ATTACK(fighter, 1, 0, Hash40::new("body"), 0.0, 361, 0, 0, 0, 3.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        if inputs {
            AttackModule::clear_all(fighter.module_accessor);
            macros::ATTACK(fighter, 1, 0, Hash40::new("body"), 0.0, 361, 0, 0, 0, 3.0, 0.0, 5.0, 10.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        }
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ILLUSION, false, -1);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specials);
}

//Can cancel with grab, attack, speciallw