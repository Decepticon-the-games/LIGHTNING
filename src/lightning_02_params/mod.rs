use smash::params::*;

fn params_main(params_info: &ParamsInfo<'_>) {

    //COMMON.PRC//
    
    if let Ok(common) = params_info.get::<CommonParams>() {

        //Wider parry window
        common.shield_just_frame = 15;

        //increase regrab time (non chaingrabbing)
        common.invalid_capture_frame = 70;

        //Dashdancing
        common.turn_dash_frame = 1;

        //Remove Knockback Untechable
        common.invalid_passive_speed = 999.0;

        //Animation between hitlag, aesthetic slow motion during hitlag (kinda like Mortal Kombat)
        common.attack_hit_slow_rate = 1.0;

        //Reduced all dodge motion rate stale
        common.escape_penalty_frame = 1;
        common.escape_penalty_recovry_frame = 1;
        common.escape_penalty_motion_rate = 0.06;
        common.escape_f_penalty_motion_rate = 0.06;
        common.escape_b_penalty_motion_rate = 0.06;

        //Increased DI
        common.damage_fly_correction_max = 18.0;

        //Shield Pressure

            //Shield depletes slower, shield HP
            common.shield_max = 100.0;

            //Shieldstun frame 2 is 3x higher 
            common.shield_setoff_add = 7.0;

            //Base shieldun multiplier is lower
            common.shield_scale_min = 0.8;
            
            //Minimum shield size is higher, 8/10 full size
            common.shield_scale_min = 0.8;

            //Shield HP after shield break is set to full size 
            common.shield_reset = 100.0;

            //Number of consecutive hits to dodge out of shield 
            common.shield_setoff_escape = 999;

            //Fames until you can grab after shieldstun
            common.shield_setoff_catch_frame = 7;

            //Max Shied Pushback is slightly raised 
            common.shield_setoff_speed_max = 1.9;

            //Attacker Pushback goes forward 
            common.shield_rebound_speed_add = -0.025;
            common.shield_rebound_speed_mul = -0.04;


        //COUNTER SHIELD PRESSURE

            //Shield tilt speed is normalized, thus faster 
            common.shield_comp_mul = 1.0;
            common.shield_comp_reach_mul = 1.0;

            //Parry frame 1 off shield stun
            common.guard_damage_just_shield_disable_frame = 0;

            //Max amount of consecutive parries?? 
            common.continue_just_shield_count = 20;

            //Shield drop cancel frame 1
            common.guard_off_cancel_frame = 1;

            //Shield stale
            common.shield_pattern_power_mul = 0.15;

        //Buffer Window
        common.precede = 7;
       
    }
    //FIGHTER_PARAM_MOTION.PRC// (doesn't seem to be supported)

    //if let Ok(fighter_param_motion) = params_info.get::<FighterParams>() {
        //Wavedashing
        //    fighter_param_motion.escape_air_slide_back_distance = 1;
        //    fighter_param_motion.escape_air_slide_back_end_frame = 0;
        //    fighter_param_motion.escape_air_slide_speed = 8.4;
        //    fighter_param_motion.escape_air_slide_penalty_speed = 8.4;
        //    fighter_param_motion.escape_air_slide_end_speed = 1;
        //    fighter_param_motion.escape_air_slide_stiff_start_frame = 26;
        //    fighter_param_motion.escape_air_slide_stick = 0.33
        //    fighter_param_motion.landing_frame_escape_air_slide_max = 10;
        
            //Longer directional airdodge distances, hence longer wavedashing and air dashing
        //    fighter_param_motion.escape_air_slide_distance = 30;
        //    fighter_param_motion.escape_air_slide_penalty_distance = 30;
    //}

    //BATTLE_OBJECT.PRC//

    //if let Ok(battle_object) = params_info.get::<BattleObjectParams>() {
    //    //More base hitlag frames, to accommodate speed so combos are more visible (pluse aesthetic)
    //    battle_object.hitstop_frame_add = 15;
    //}
}

// in main

pub fn install() {
    smash::params::add_hook(params_main).unwrap();
}