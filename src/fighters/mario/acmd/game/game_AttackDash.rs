use super::*;
#[acmd_script( agent = "mario", script = "game_attackdash", category = ACMD_GAME )]
    unsafe fn game_attackdash(fighter: &mut L2CAgentBase) {
        let lua_state = fighter.lua_state_agent;
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 50, 43, 0, 100, 3.5, 0.0, 1.5, 5.4, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.875);
            FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4.0, 6.0);
            enable_attack_cancel(fighter);
    }

        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 80, 30, 0, 30, 2.7, 0.0, 1.5, 4.9, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.875);
            enable_attack_cancel(fighter);
    }

        wait(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            whiff_cancel(fighter); 
        }
        frame(fighter.lua_state_agent, 32.0);
        if macros::is_excute(fighter) {
            FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4.0, 4.0);
        }
        frame(fighter.lua_state_agent, 41.0);
        if macros::is_excute(fighter) {
            FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.0, 3.0);
        }
    }
    pub fn install() {
        smashline::install_acmd_scripts!(
        game_attackdash);
    }