use super::*;
#[acmd_script( agent = "tantan", script = "game_attacklongpulllb3", category = ACMD_GAME, low_priority )]
unsafe fn game_attacklongpulllb3(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_BLOCKED_PUNCH_L) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_DRAGONIZE_L) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_REINFORCE_L) {
                frame(fighter.lua_state_agent, 1.0);
                if macros::is_excute(fighter) {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 15.0, 361, 75, 0, 40, 5.5, 4.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
                    AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
                    ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_dragonarm"), 9, false, *BATTLE_OBJECT_ID_INVALID as u32);
                }
                frame(fighter.lua_state_agent, 2.0);
                methodlib::L2CValue::operatorbool()const(is_excute);
            }
            else{
            if WorkModule::is_flag(fighter.module_accessor, *LUA_SCRIPT_LINE_MAX) {
                frame(fighter.lua_state_agent, 1.0);
                if macros::is_excute(fighter) {
                    macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 13.8, 361, 75, 0, 40, 5.5, 4.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
                    AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
                    ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_dragonarm"), 9, false, *BATTLE_OBJECT_ID_INVALID as u32);
                }
                frame(fighter.lua_state_agent, 2.0);
                methodlib::L2CValue::operatorbool()const(is_excute);
            }
            else{
            frame(fighter.lua_state_agent, 1.0);
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 8.0, 361, 90, 0, 50, 5.5, 4.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
                AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
                ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_dragonarm"), 9, false, *BATTLE_OBJECT_ID_INVALID as u32);
            }
            frame(fighter.lua_state_agent, 2.0);
            methodlib::L2CValue::operatorbool()const(is_excute);
        }
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_);
}
AttackModule::clear_all(fighter.module_accessor);
whiff_cancel(fighter);
}
}
}