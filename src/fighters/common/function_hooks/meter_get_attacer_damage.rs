#[skyline::hook(offset = 0x6310a0())]
unsafe fn fighter_handle_damage_hook(fighter: *mut smash::app::BattleObject, arg: *const u8) {
    let module_accessor = (*fighter).module_accessor;
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let damage_received = WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE);
    call_original!(fighter, arg);
    let damage_received = WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE) - damage_received;
    let attacker_ids = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SUCCEED_ATTACKER_ENTRY_ID);
    for x in 0..8 {
        if attacker_ids & (1 << x) == 0 {
            continue;
        }
        if let Some(object_id) = crate::util::get_active_battle_object_id_from_entry_id(x) {
            let object = crate::util::get_battle_object_from_id(object_id);
            if !object.is_null() && super::is_hdr_object((*object).vtable as _) {
                VarModule::set_float(object, vars::common::instance::LAST_ATTACK_DAMAGE_DEALT, damage_received);
                VarModule::set_int(object, vars::common::instance::LAST_ATTACK_RECEIVER_ENTRY_ID, (*fighter).battle_object_id as i32);
                MeterModule::signal_hit(object);
            }
        }
    }
}