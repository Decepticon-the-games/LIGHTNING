#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]
use {
    smash::{
        lua2cpp::{L2CAgentBase,L2CFighterCommon,L2CFighterBase},
        phx::{Hash40,Vector3f,Vector2f},
        hash40,
        app::{lua_bind::*, sv_animcmd::*,*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};
use crate::fighters::{
    common::{
        mechanics::{
            cancels::{
                attack_cancels::{
                    ATTACK_CANCEL, ENABLE_ATTACK_CANCEL, ENABLE_MULTIHIT_CANCEL, ENABLE_MULTIHIT_COUNT, MULTIHIT, MULTIHIT_COUNT, IS_SUCCESSFUL_HIT, 
                    cancel_on_hit::{is_attack_cancel, multihit_cancel, multihit_counter, cancel_on_hit, is_after_hitlag}
                },
                motioncancels::{
                    CANCEL_IN_NEUTRAL, AIRDASH, WAVEDASH, WAVESTEP, AIRSTEP, AIRDODGE_PLUS, PROJECTILE_COUNTER, PROJECTILE_COUNT, DISABLE_MOVESET_TRANSITIONS
                },
                counter_cancels::COUNTER_CANCEL
            },
            lightning_mechanics::{
                //crimson_cancel::CRIMSON_CANCEL,
                lightning_mode::LIGHTNING,
                vanish::{
                    VANISH, CAN_VANISH, CANCEL_INTO_VANISH, ENABLE_CANCEL_INTO_VANISH,
                    opff::{float,cancel_into_vanish, enable_vanish_effects, disable_vanish_effects, is_cancel_into_vanish_conditions}
                }
            },
            misc::{

            }
        },
        function_hooks::notify_collision_event::{PROJECTILE_HIT, DIRECT_HIT, IS_HIT}
    }
};
///*
pub mod common;
pub mod mario;
pub mod donkey;
pub mod link;
pub mod samus;
pub mod samusd;
pub mod yoshi;
pub mod kirby;
pub mod fox;
pub mod pikachu;
pub mod luigi;
pub mod ness;
pub mod captain;
pub mod purin;

pub mod peach;
pub mod daisy;
pub mod koopa;
pub mod popo;
pub mod nana;
pub mod sheik;
pub mod zelda;
pub mod mariod;
pub mod pichu;
pub mod falco;
pub mod marth;
pub mod lucina;
pub mod younglink;
pub mod ganon;
pub mod mewtwo;
pub mod roy;
pub mod chrom;
pub mod gamewatch;

pub mod metaknight;
pub mod pit;
pub mod pitb;
pub mod szerosuit;
pub mod wario;
pub mod snake;
pub mod ike;
pub mod pzenigame;
pub mod pfushigisou;
pub mod plizardon;
pub mod diddy;
pub mod lucas;
pub mod sonic;
pub mod dedede;
pub mod pikmin;
pub mod lucario;
pub mod robot;
pub mod toonlink;
pub mod wolf;

pub mod murabito;
pub mod rockman;
pub mod wiifit;
pub mod rosetta;
pub mod littlemac;
pub mod gekkouga;
pub mod palutena;
pub mod pacman;
pub mod reflet;
pub mod shulk;
pub mod koopajr;
pub mod duckhunt;
pub mod ryu;
pub mod ken;
pub mod cloud;
pub mod kamui;
pub mod bayonetta;

pub mod inkling;
pub mod ridley;
pub mod simon;
pub mod richter;
pub mod krool;
pub mod shizue;
pub mod gaogaen;
pub mod packun;
pub mod jack;
pub mod brave;
pub mod buddy;
pub mod dolly;
pub mod master;
pub mod tantan;
pub mod pickel;
pub mod edge;
pub mod eflame;
//pub mod element;
pub mod elight;
pub mod demon;
pub mod trail;
pub mod miifighter;
pub mod miiswordsman;
pub mod miigunner;



pub fn install() {

common::install();
mario::install();
donkey::install();
link::install();
samus::install();
samusd::install();
yoshi::install();
kirby::install();
fox::install();
pikachu::install();
luigi::install();
ness::install();
captain::install();
purin::install();

peach::install();
daisy::install();
koopa::install();
popo::install();
nana::install();
sheik::install();
zelda::install();
mariod::install();
pichu::install();
falco::install();
marth::install();
lucina::install();
younglink::install();
ganon::install();
mewtwo::install();
roy::install();
chrom::install();
gamewatch::install();

metaknight::install();
pit::install();
pitb::install();
szerosuit::install();
wario::install();
snake::install();
ike::install();
pzenigame::install();
pfushigisou::install();
plizardon::install();
diddy::install();
lucas::install();
sonic::install();
dedede::install();
pikmin::install();
lucario::install();
robot::install();
toonlink::install();
wolf::install();

murabito::install();
rockman::install();
wiifit::install();
rosetta::install();
littlemac::install();
gekkouga::install();
palutena::install();
pacman::install();
reflet::install();
shulk::install();
koopajr::install();
duckhunt::install();
ryu::install();
ken::install();
cloud::install();
kamui::install();
bayonetta::install();

inkling::install();
ridley::install();
simon::install();
richter::install();
krool::install();
shizue::install();
gaogaen::install();
packun::install();
jack::install();
brave::install();
buddy::install();
dolly::install();
master::install();
tantan::install();
pickel::install();
edge::install();
eflame::install();
//element::install();
elight::install();
demon::install();
trail::install();
miifighter::install();
miiswordsman::install();
miigunner::install();
}
//*/


/*
//TESTING
pub mod common;
pub mod fox;
pub mod edge;
pub mod falco;

pub fn install() {
common::install();
fox::install();
edge::install();
falco::install();
}
*/
