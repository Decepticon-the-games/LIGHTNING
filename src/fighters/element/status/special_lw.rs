use smash::lib::{L2CValue,L2CAgent,lua_const::*};
use smash::lua2cpp::{L2CAgentBase,L2CFighterCommon,L2CFighterBase};
use smash::phx::*;
use smash::hash40;
use smash::app::lua_bind::*;
use smash::app::*;
use smash_script::macros::*;
use smash_script::lua_args;
use smashline::*;

pub static mut FIGHTER_MANAGER : usize = 0;
pub static mut SWAP : [bool; 8] = [false; 8];

pub unsafe fn swap_aegis(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    SWAP[entry_id] == false;

    element_special_lw_main(fighter);

}

extern "C" {
    #[link_name = "\u{1}_ZN3app29FighterElementLinkEventChange13new_l2c_tableEv"]
    pub fn FighterElementLinkEventChange__new_l2c_table() -> smash::lib::L2CValue;
}

pub unsafe fn element_special_lw_main(fighter: &mut L2CFighterCommon) {
    let mut event = FighterElementLinkEventChange__new_l2c_table();
    event["link_event_kind_"].assign(&L2CValue::new_int(0x1cd83c14e3u64));
    event["object_id_"].assign(&L2CValue::I32(*BATTLE_OBJECT_ID_INVALID));
    let callable: extern "C" fn() -> *mut smash::app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
    let link_event = callable();
    lua_bind::LinkEvent::load_from_l2c_table(link_event,&event);
    LinkModule::send_event_nodes_struct(fighter.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, link_event, 0);
    lua_bind::LinkEvent::store_l2c_table(link_event);
    let deleter: extern "C" fn(*mut smash::app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
    deleter(link_event);
    let callable: extern "C" fn() -> *mut smash::app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
    let link_event = callable();
    lua_bind::LinkEvent::load_from_l2c_table(link_event, &event);
    LinkModule::send_event_nodes_struct(fighter.module_accessor, *ITEM_LINK_NO_HAVE, link_event, 0);
    lua_bind::LinkEvent::store_l2c_table(link_event);
    let deleter: extern "C" fn(*mut smash::app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
    deleter(link_event);
    let entry_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    let fighter_info = lua_bind::FighterManager::get_fighter_information(fighter_manager,FighterEntryID(entry_id));
    if lua_bind::FighterInformation::is_rabbit_cap(fighter_info) {
        ItemModule::eject_attach(fighter.module_accessor, ItemKind(*ITEM_KIND_USAGIHAT),  true, true);
    }
    if lua_bind::FighterInformation::is_reflector(fighter_info) {
        ItemModule::eject_attach(fighter.module_accessor, ItemKind(*ITEM_KIND_BADGE), true, true);
    }
    if lua_bind::FighterInformation::is_rocketbelt(fighter_info) {
        ItemModule::eject_attach(fighter.module_accessor, ItemKind(*ITEM_KIND_ROCKETBELT), true, true);
    }
    if lua_bind::FighterInformation::is_screw(fighter_info) {
        ItemModule::eject_attach(fighter.module_accessor, ItemKind(*ITEM_KIND_SCREW), true, true);
    }
    if lua_bind::FighterInformation::is_backshield(fighter_info) {
        ItemModule::eject_attach(fighter.module_accessor, ItemKind(*ITEM_KIND_BACKSHIELD), true, true);
    }
    AreaModule::set_whole(fighter.module_accessor,false);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main_loop as *const () as _))
}