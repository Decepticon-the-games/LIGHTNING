use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smash::lib::lua_const::*;
use smashline::*;
use smash_script::*;
use smash::phx::Hash40;
use smash::app::sv_animcmd::*;

// Use this for general per-frame fighter-level hooks
#[fighter_frame( agent = FIGHTER_KIND_CLOUD )]
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
        || status_kind == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2
        || status_kind == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 
        || ! status_kind == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2
        || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {   
            
            //FIX Side Special cancel
            if status_kind == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3 {

                if MotionModule::frame(module_accessor) >=25.0  {                       
                    if AttackModule:: is_attack_occur(module_accessor){
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }
            // SIDE SMASH 
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
                //CANCEL WITH DOWN SMASH AND SIDE B
                if AttackModule:: is_attack_occur(module_accessor){
                    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 
                    || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) !=0 {
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
                //FIX Side Smash cancel
                if MotionModule::frame(module_accessor) >=28.0  {                       
                    if AttackModule:: is_attack_occur(module_accessor){
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }
            //FIX UP B
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
               
                if MotionModule::frame(module_accessor) >= 28.0 {
                    CancelModule::enable_cancel(module_accessor); 
                }
            }
        }
        else if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
        && ! (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4)
        && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3)
        && ! (status_kind == *FIGHTER_STATUS_KIND_THROW) {
            if AttackModule:: is_attack_occur(fighter.module_accessor) && ! SlowModule::is_slow(module_accessor){
                CancelModule::enable_cancel(module_accessor);
            }
        }
    }                                      
}
//Make Aerial Side Special 2 link with 3 faster
#[acmd_script( agent = "cloud", script = "game_specialairs2", category = ACMD_GAME, low_priority )]
unsafe fn cloud_side_b_connect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_DETACH_EFFECT);
    }
    frame(fighter.lua_state_agent, 2.0);
    //get_value_float(SO_VAR_FLOAT_LR)
    //if(0x180150(2001240, 0)){
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 75, 25, 0, 33, 7.3, 0.0, 13.5, 17.7, Some(0.0), Some(9.0), Some(15.7), 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 60, 25, 0, 43, 3.0, 0.0, 8.5, 9.5, Some(0.0), Some(8.5), Some(9.5), 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    // else{
    //     if(is_excute){
    //         ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=75, KBG=25, FKB=0, BKB=33, Size=7.3, X=0.0, Y=13.5, Z=15.7, X2=0.0, Y2=9.0, Z2=17.7, Hitlag=1.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
    //         ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=60, KBG=25, FKB=0, BKB=43, Size=3.0, X=0.0, Y=8.5, Z=9.5, X2=0.0, Y2=8.5, Z2=9.5, Hitlag=1.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
    //     }
    // }
    //}
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

pub fn install() {
    smashline::install_agent_frames!(once_per_fighter_frame);
    smashline::install_acmd_scripts!(cloud_side_b_connect);
}