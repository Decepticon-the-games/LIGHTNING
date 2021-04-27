use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};



//FALCO 
//-------------------------------------------------------------

//Detector hitbox on side b
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_FALCO, 
    animation = "special_air_s",
    animcmd = "game_specialairs")]
pub fn falco_side_b_air(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        if(is_excute) {
            JostleModule::set_status(false)
            
        }
        frame(Frame=2)
        if(is_excute) {
            ArticleModule::generate_article(FIGHTER_FALCO_GENERATE_ARTICLE_ILLUSION,false,0)
            
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=0, FKB=0, BKB=0, Size=4.0, X=0.0, Y=10.0, Z=8.8, X2=0.0, Y2=5.0, Z2=8.8, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_search"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=0, FKB=0, BKB=0, Size=4.0, X=0.0, Y=12.0, Z=8.8, X2=0.0, Y2=5.0, Z2=8.8, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_search"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            }
        frame(Frame=4)
        if(is_excute) {
            ArticleModule::generate_article(FIGHTER_FALCO_GENERATE_ARTICLE_ILLUSION,false,0)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_FALCO, 
    animation = "special_s",
    animcmd = "game_specials")]
pub fn falco_side_b(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        if(is_excute) {
            JostleModule::set_status(false)
            
        }
        frame(Frame=2)
        if(is_excute) {
            ArticleModule::generate_article(FIGHTER_FALCO_GENERATE_ARTICLE_ILLUSION,false,0)
            
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=0, FKB=0, BKB=0, Size=4.0, X=0.0, Y=9.0, Z=8.8, X2=0.0, Y2=5.0, Z2=8.8, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_search"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.0, Angle=361, KBG=0, FKB=0, BKB=0, Size=4.0, X=0.0, Y=12.0, Z=8.8, X2=0.0, Y2=5.0, Z2=8.8, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_NO_FLOOR, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_search"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
        }
        frame(Frame=4)
        if(is_excute) {
            ArticleModule::generate_article(FIGHTER_FALCO_GENERATE_ARTICLE_ILLUSION,false,0)
        }
    });
}
//SHIEK??
//-------------------------------------------------------------------------------------------------------
//Up b second hitbox overlapped with a bigger detector hitbox

//ZELDA
//--------------------------------------------------------------------------------------------------------
//Up b second hitbox overlapped with a bigger detector hitbox

//MEWTWO
//--------------------------------------------------------------------------------------------------------
//Up b detector hitbox when teleported

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MEWTWO, 
    animation = "special_hi",
    animcmd = "game_specialhi")]
pub fn mewtwo_up_b(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        if(is_excute) {
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.0, Angle=367, KBG=100, FKB=0, BKB=0, Size=12.0, X=0.0, Y=10.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
        }
    });
}
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MEWTWO, 
    animation = "special_air_hi",
    animcmd = "game_specialairhi")]
pub fn mewtwo_up_b_air(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        if(is_excute) {
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=367, KBG=100, FKB=50, BKB=0, Size=12.0, X=0.0, Y=10.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
        }
    });
}
//PIT/DARK PIT
//----------------------------------------------------------------------------------------------------------
//Up b detector hitbox starting from flying
//DOES NOT WORK*******************


//LUCARIO?? 
//----------------------------------------------------------------------------------------------------------
//Change Up b  hitbox starting from flying


//PALUTENA
//--------------------------------------------------------------------------------------------------------
//Up b detector hitbox when teleported

//SEPHIROTH
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_EDGE, 
    animation = "special_hi1",
    animcmd = "game_specialhi1")]
pub fn seph_up_b_move(fighter: &mut L2CFighterCommon) {
    acmd!(lua_state,{
        
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=367, KBG=100, FKB=50, BKB=0, Size=12.0, X=0.0, Y=10.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
    
    });
}


pub fn install() {
    acmd::add_hooks!(
        falco_side_b_air,
        falco_side_b,
        mewtwo_up_b,
        mewtwo_up_b_air,
        //seph_up_b_move
        
    );
}
