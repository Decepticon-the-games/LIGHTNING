use super::*;
#[acmd_script( agent = "dedede", script = "game_specialnshotobjectswallow", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnshotobjectswallow(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    macros::FT_MOTION_RATE(fighter, 0.6);
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 14.7, 4);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(fighter) {
            FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4, 4);
        }
    }
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_);
}
