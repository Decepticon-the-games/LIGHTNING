use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lib::lua_const::*;
use acmd::*;
use smash::hash40;

// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        //let situation_kind = smash::app::lua_bind::StatusModule::situation_kind(module_accessor);
        let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
        
    
        if fighter_kind == *FIGHTER_KIND_CLOUD {
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
            || status_kind == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2
            || status_kind == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3
            || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {   
                
                //FIX Side Special
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
                    //FIX Side Smash
                    if MotionModule::frame(module_accessor) >=28.0  {                       
                        if AttackModule:: is_attack_occur(module_accessor){
                            CancelModule::enable_cancel(module_accessor);
                        }
                    }
                }
            }
            else if ! (status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK)
            && ! (status_kind == *FIGHTER_STATUS_KIND_ATTACK_100) {
                if AttackModule:: is_attack_occur(module_accessor) {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
            
        }
    }                                      
}
//Make Side Special 2 link with 3 faster
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CLOUD, 
    animation = "special_air_s2",
    animcmd = "game_specialairs2")]
pub fn cloud_side_b_connect(fighter: &mut L2CFighterCommon) {
    acmd!(lua_state,{
        
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_INPUT_CHECK)
            WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_DETACH_EFFECT)
        }
        frame(Frame=2)
        //get_value_float(SO_VAR_FLOAT_LR)
        //if(0x180150(2001240, 0)){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=95, KBG=25, FKB=0, BKB=33, Size=7.3, X=0.0, Y=13.5, Z=17.7, X2=0.0, Y2=9.0, Z2=15.7, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=60, KBG=25, FKB=0, BKB=43, Size=3.0, X=0.0, Y=8.5, Z=9.5, X2=0.0, Y2=8.5, Z2=9.5, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            else{
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=95, KBG=25, FKB=0, BKB=33, Size=7.3, X=0.0, Y=13.5, Z=15.7, X2=0.0, Y2=9.0, Z2=17.7, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=60, KBG=25, FKB=0, BKB=43, Size=3.0, X=0.0, Y=8.5, Z=9.5, X2=0.0, Y2=8.5, Z2=9.5, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                }
            }
        //}
        frame(Frame=4)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=7)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE)
        }
        frame(Frame=20)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_CLOUD_STATUS_SPECIAL_S_FLAG_INPUT_CHECK)
        }

    });

}

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
    acmd::add_hooks!(cloud_side_b_connect);
}