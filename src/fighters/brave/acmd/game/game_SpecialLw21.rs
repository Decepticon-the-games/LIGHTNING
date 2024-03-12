use super::*;
#[acmd_script( agent = "brave", script = "game_speciallw21", category = ACMD_GAME, low_priority )]
unsafe fn game_speciallw21(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        whiff_cancel(fighter);
        //battle_object();
        //methodlib::L2CValue::L2CValue(void*)();
    }
    else{
    //methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
}
//methodlib::L2CValue::as_pointer()const();
let battle_object = smash::app::sv_system::battle_object(fighter.lua_state_agent);
let instance = std::mem::transmute::<&mut smash::app::BattleObject, &mut smash::app::Fighter>(battle_object);
special_lw_active_command(instance);;
}    
pub fn install() {
    smashline::install_acmd_scripts!(
    game_speciallw21);
}