use super::*;

pub unsafe fn shields_dyn_int_param(param_type: u64, param_hash: u64) -> Option<i32> {
    if param_type == hash40("common") {
        if param_hash == hash40("shield_setoff_catch_frame") {//Frames until shield grab is possible after shield stun
            return Some(7);
        }
        
        if param_hash == hash40("shield_setoff_escape") {//Number of consecutive hits during shieldstun to cancel with dodges
            return Some(5);
        }
        
        if param_hash == hash40("guard_off_cancel_frame") {//Move out of shield frame 1
            return Some(1);
        }
        if param_hash == hash40("guard_off_enable_shield_frame") {//Minimum time between shields (frames)
            return Some(30);
        }
    }
    None
}

pub unsafe fn shields_dyn_float_param(param_type: u64, param_hash: u64) -> Option<f32> {

    if param_type == hash40("common") {
    
        if param_hash == hash40("shield_max") {//2x lifespan (100)
        return  Some(100.0);
        }

        if param_hash == hash40("shield_scale_min") {//Minimum shield size
            return  Some(0.8);
        }

        if param_hash == hash40("shield_scale_mul") {//shield size multiplier
         
            return  Some(1.0);
        }

        if param_hash == hash40("shield_setoff_add") {//Base shieldstun frames
            return  Some(0.0);
        }

        if param_hash == hash40("shield_setoff_mul") {//Base shieldstun multiplier
            return  Some(1.0);
        }

        if param_hash == hash40("shield_setoff_mul_fighter_shot") {//Shieldstun multiplier for projectile attacks
            return  Some(1.0);
        }

        if param_hash == hash40("shield_stiff_mul_attack_4") {//Shieldstun multiplier for smash attacks
            return  Some(1.0);
        }

        if param_hash == hash40("shield_stiff_mul_attack_air") {//Shieldstun multiplier for aerial attacks
            return  Some(1.0);
        }

        if param_hash == hash40("shield_setoff_speed_mul") {//shield pushback multiplier
            return  Some(0.1);
        }

        if param_hash == hash40("shield_setoff_speed_max") {//Maximum shield pushback is slightly raised
            return  Some(1.9);
        }

        if param_hash == hash40("shield_pattern_power_mul") {//Stale factor multiplier with hitting shield, drastically decreased
            return  Some(0.15);
        }

        if param_hash == hash40("shield_reset") {//life after shield break recovery (100)
            return  Some(100.0);
        }

        if param_hash == hash40("shield_comp_mul") {//shield tilt speed normalized
            return  Some(1.0);
        }

        if param_hash == hash40("shield_comp_reach_mul") {//shield tilt speed normalized
            return  Some(1.0);
        }
    }
    None
}
