use super::*;

#[fighter_frame_callback]
pub fn prints(fighter : &mut L2CFighterCommon) {
    unsafe{
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);

        if entry_id <1 {

            //println!("is_attcan: {}", is_attack_cancel(fighter, state, next_input != 0, prev_state));
            //println!("can_vanish: {}", is_cancel_into_vanish_conditions(fighter, false, !(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0, false));
            //println!("en_multihitcancel: {}", ENABLE_MULTIHIT_CANCEL[entry_id]);

            //println!("en_attack_cancel: {}", ENABLE_ATTACK_CANCEL[entry_id]);
            //println!("multihitcount: {}", MULTIHIT_COUNT[entry_id]);
            //println!("is_hit: {}", AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) );
            
            //println!("is_enable_cancel: {}", CancelModule::is_enable_cancel(fighter.module_accessor) );
            //println!("cancein_neutral: {}", CANCEL_IN_NEUTRAL[entry_id]);
            //println!("phit: {}", PROJECTILE_HIT[entry_id] );
            //println!("cancel2vanish: {}", ENABLE_CANCEL_INTO_VANISH[entry_id] );
            //println!("pos_x: {}", PostureModule::pos_x(fighter.module_accessor) );
            //println!("pos_y: {}", PostureModule::pos_y(fighter.module_accessor) );
            

        }
    }
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(prints);
}