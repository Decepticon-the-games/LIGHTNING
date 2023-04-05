use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::{Hash40,Vector3f},
        app::{lua_bind::*, sv_animcmd::*,*},
        lib::lua_const::*,
    },
    smash_script::*,
    smashline::*
};
use crate::fighters::{
    common::{
        mechanics::{
            attack_cancels::{
                ENABLE_ATTACK_CANCEL,ENABLE_MULTIHIT_CANCEL,MOVEMENT_CANCEL
            },
            motioncancels::{
                CANCEL_IN_NEUTRAL
            }
        }
    }
};

//static entry_id: WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

pub mod common;
pub mod bayonetta;
pub mod brave;
pub mod buddy;
pub mod captain;
pub mod cloud;
pub mod dedede;
pub mod demon;
pub mod diddy;
pub mod dolly;
pub mod donkey;
pub mod duckhunt;
pub mod edge;
pub mod eflame;
pub mod elight;
pub mod falco;
pub mod fox;
pub mod gamewatch;
pub mod ganon;
pub mod gaogaen;
pub mod gekkouga;
pub mod ike;
pub mod inkling;
pub mod jack;
pub mod kamui;
pub mod kirby;
pub mod koopa;
pub mod koopajr;
pub mod krool;
pub mod link;
pub mod littlemac;
pub mod lucario;
pub mod lucas;
pub mod luigi;
pub mod mario;
pub mod mariod;
pub mod marth;
pub mod lucina;
pub mod master;
pub mod metaknight;
pub mod mewtwo;
pub mod miifighter;
pub mod miiswordsman;
pub mod miigunner;
pub mod murabito;
pub mod ness;
pub mod packun;
pub mod pacman;
pub mod palutena;
pub mod peach;
pub mod daisy;
pub mod pfushigisou;
pub mod pichu;
pub mod pikachu;
pub mod pickel;
pub mod pikmin;
pub mod pit;
pub mod pitb;
pub mod plizardon;
pub mod popo;
pub mod nana;
pub mod purin;
pub mod pzenigame;
pub mod reflet;
pub mod ridley;
pub mod robot;
pub mod rockman;
pub mod rosetta;
pub mod roy;
pub mod chrom;
pub mod ryu;
pub mod ken;
pub mod samus;
pub mod samusd;
pub mod sheik;
pub mod shizue;
pub mod shulk;
pub mod simon;
pub mod richter;
pub mod snake;
pub mod sonic;
pub mod szerosuit;
pub mod tantan;
pub mod trail;
pub mod toonlink;
pub mod wario;
pub mod wiifit;
pub mod wolf;
pub mod yoshi;
pub mod younglink;
pub mod zelda;



pub fn install() {
    common::install();
    bayonetta::install();
    brave::install();
    buddy::install();
    captain::install();
    cloud::install();
    dedede::install();
    demon::install();
    diddy::install();
    dolly::install();
    donkey::install();
    duckhunt::install();
    edge::install();
    eflame::install();
    elight::install();
    falco::install();
    fox::install();
    gamewatch::install();
    ganon::install();
    gaogaen::install();
    gekkouga::install();
    ike::install();
    inkling::install();
    jack::install();
    kamui::install();
    kirby::install();
    koopa::install();
    koopajr::install();
    krool::install();
    link::install();
    littlemac::install();
    lucario::install();
    lucas::install();
    luigi::install();
    mario::install();
    mariod::install();
    marth::install();
    lucina::install();
    master::install();
    metaknight::install();
    mewtwo::install();
    miifighter::install();
    miiswordsman::install();
    miigunner::install();
    murabito::install();
    ness::install();
    packun::install();
    pacman::install();
    palutena::install();
    peach::install();
    daisy::install();
    pfushigisou::install();
    pichu::install();
    pikachu::install();
    pickel::install();
    pikmin::install();
    pit::install();
    pitb::install();
    plizardon::install();
    popo::install();
    nana::install();
    purin::install();
    pzenigame::install();
    reflet::install();
    ridley::install();
    robot::install();
    rockman::install();
    rosetta::install();
    roy::install();
    chrom::install();
    ryu::install();
    ken::install();
    samus::install();
    samusd::install();
    sheik::install();
    shizue::install();
    shulk::install();
    simon::install();
    richter::install();
    snake::install();
    sonic::install();
    szerosuit::install();
    tantan::install();
    toonlink::install();
    trail::install();
    wario::install();
    wiifit::install();
    wolf::install();
    yoshi::install();
    younglink::install();
    zelda::install();

}