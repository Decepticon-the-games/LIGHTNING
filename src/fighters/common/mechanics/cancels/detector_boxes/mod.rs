use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;

use smash_script::*;
use smash::app::sv_animcmd::*;

//FALCO 
//-------------------------------------------------------------

//Detector hitbox on side b
/*#[acmd_script( agent = "falco", scripts = ["game_specials", "game_specialairs"], category = ACMD_GAME, low_priority )]
unsafe fn falco_side_b(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_ILLUSION, false, 0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_ILLUSION, false, 0);
    }
}*/
//SHIEK??
//-------------------------------------------------------------------------------------------------------
//Up b second hitbox overlapped with a bigger detector hitbox



//ZELDA
//--------------------------------------------------------------------------------------------------------
//Up b second hitbox overlapped with a bigger detector hitbox

//MEWTWO
//--------------------------------------------------------------------------------------------------------
//Up b detector hitbox when teleported

#[acmd_script( agent = "mewtwo", scripts = ["game_specialhi", "game_specialairhi"], category = ACMD_GAME, low_priority )]
unsafe fn mewtwo_up_b(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 12.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}

//DIDDY KONG
//----------------------------------------------------------------------------------------------------------
//Up b detector hitbox starting from flying



//PIT/DARK PIT
//----------------------------------------------------------------------------------------------------------
//Up b detector hitbox starting from flying
#[acmd_script( agent = "pit", scripts = ["game_specialhi", "game_specialairhi"], category = ACMD_GAME, low_priority )]
unsafe fn pit_ub_b(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.0, 0.0, 9.0, 8.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.0, 0.0, 12.0, 8.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}

#[acmd_script( agent = "pitb", scripts = ["game_specialhi", "game_specialairhi"], category = ACMD_GAME, low_priority )]
unsafe fn darkpit_up_b(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.0, 0.0, 9.0, 8.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.0, 0.0, 12.0, 8.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}


//LUCARIO?? 
//----------------------------------------------------------------------------------------------------------
//Change Up b  hitbox starting from flying


//PALUTENA
//--------------------------------------------------------------------------------------------------------
//Up b detector hitbox when teleported

//INKLING
//--------------------------------------------------------------------------------------------------------
#[acmd_script( agent = "inkling", script = "game_specialhijump", category = ACMD_GAME, low_priority )]
unsafe fn inkling_up_b(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 367, 100, 50, 0, 12.0, 0.0, 10.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}

//SEPHIROTH
//--------------------------------------------------------------------------------------------------------
//#[acmd_func(
//    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
//    battle_object_kind = FIGHTER_KIND_EDGE, 
//    animation = "special_hi1",
//    animcmd = "game_specialhi1")]
//pub fn seph_up_b_move(fighter: &mut L2CFighterCommon) {
//    acmd!(lua_state,{
//        
//        ATTACK(ID=1, 0, Hash40::new("top"), 1.0, 367, 100, 50, 0, 12.0, 0.0, 10.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE)
    
//    });
//}


pub fn install() {
    smashline::install_acmd_scripts!(
        //falco_side_b,
        mewtwo_up_b,
        //seph_up_b_move,
        
        inkling_up_b,
        pit_ub_b,
        darkpit_up_b,  
    );
}
