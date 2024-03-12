use super::*;


#[fighter_frame_callback]
pub fn prints(fighter : &mut L2CFighterCommon) {
    unsafe{
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let opponent_boma = sv_battle_object::module_accessor(WHO_GOT_HIT_BOMA[entry_id]);

        if entry_id ==1 {

            //println!("is_attcan: {}", is_attack_cancel(fighter, state, next_input != 0, prev_state));
            //println!("can_vanish: {}", is_cancel_into_vanish_conditions(fighter, false, !(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0, false));
            //println!("en_multihitcancel: {}", ENABLE_MULTIHIT_CANCEL[entry_id]);

            //println!("en_attack_cancel: {}", ENABLE_ATTACK_CANCEL[entry_id]);
            //println!("attack_cancel: {}", ATTACK_CANCEL[entry_id]);
            //println!("multihitcount: {}", MULTIHIT_COUNT[entry_id]);
            //println!("is_hit: {}", AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) );
            //println!("is_enable_cancel: {}", CancelModule::is_enable_cancel(fighter.module_accessor));
            //println!("hitstop_frame: {}", SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) );
            //println!("airstep: {}", AIRSTEP[entry_id] );
            //println!("whiff2dash: {}", WHIFF_CANCEL_INTO_DASH[entry_id] );
            //println!("jumpsused: {}", WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT));
            //println!("cancein_neutral: {}", CANCEL_IN_NEUTRAL[entry_id]);
            //println!("opp_kb_speed: {}", DamageModule::reaction(opponent_boma, 0));
            //println!("guardcancel: {}", WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_GUARD_FRAME) );
            //println!("phit: {}", PROJECTILE_HIT[entry_id] );
            //println!("cancel2vanish: {}", FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_VANISH[entry_id]);
            //println!("vd: {}", DEFENDER_VANISH[entry_id] );
            //println!("pos_x: {}", PostureModule::pos_x(fighter.module_accessor) );
            //println!("pos_y: {}", PostureModule::pos_y(fighter.module_accessor) );
            //println!("degrees: {}", ControlModule::get_stick_angle(fighter.module_accessor).to_degrees());
            //println!("gauge: {}", WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHARGE_FINAL_GAUGE));
            //println!("add_damage: {}", WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHARGE_FINAL_ADD_DAMAGE));
            //println!("add_gauge_counter: {}", WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CHARGE_FINAL_ADD_GAUGE_COUNTER));
            //println!("reserve: {}", WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_ONE_MORE_CHARGE_FINAL));
        }
    }
}
pub fn install() {
    smashline::install_agent_frame_callbacks!(prints);
}