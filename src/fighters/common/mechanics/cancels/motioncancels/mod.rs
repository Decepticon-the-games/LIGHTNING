use super::*;

pub static mut CANCEL_IN_NEUTRAL : [bool; 8] = [false; 8];
pub static mut AIRDASH : [bool; 8] = [false; 8];
pub static mut WAVEDASH : [bool; 8] = [false; 8];
pub static mut WAVEDASH_MAGNET : [bool; 8] = [false; 8];
pub static mut WAVESTEP : [bool; 8] = [false; 8];
pub static mut AIRDODGE_PLUS : [bool; 8] = [false; 8];
pub static mut AIRSTEP : [bool; 8] = [false; 8];
pub static mut AIRSTEP_BUTTON: [bool; 8] = [false; 8];
static mut AIRDODGE_BUTTON : [bool; 8] = [false; 8];// for only running the code within it 1 frame.
static mut AIRDODGE_COUNT : [i32; 8] = [0; 8]; //  You start off with one airdodge. Every other airdodge after that before touching the ground increases the number up to how many jumps that fighter has.
pub static mut PROJECTILE_COUNTER : [bool; 8] = [false; 8];
pub static mut PROJECTILE_COUNT : [i32; 8] = [0; 8];
pub static mut WHIFF_CANCEL : [bool; 8] = [false; 8];
pub static mut WHIFF_CANCEL_INTO_DASH : [bool; 8] = [false; 8];//Dash cancels into most things frame 1, so side moves can be canceled back to back. This removes the transition terms for all moves cancellabe out of dash to let the initial dash animation play out, before being able to cancel into them.
pub static mut WHIFF_CANCEL_INTO_SHIELD : [bool; 8] = [false; 8];
pub static mut WHIFF_CANCEL_INTO_ESCAPE : [bool; 8] = [false; 8];
pub static mut WHIFF_CANCEL_INTO_AIRDODGE : [bool; 8] = [false; 8];
pub static mut WHIFF_CANCEL_INTO_JUMP : [bool; 8] = [false; 8];

pub static mut WHIFF_CANCEL_EFFECTS : [bool; 8] = [false; 8];//Effects only

pub mod airdash;
pub mod airstep;
pub mod cancel_in_neutral;
pub mod multiple_airdodges;
pub mod resets_falses;
//pub mod transitions;
pub mod wavedash;
//pub mod wavestep;

pub fn install() {
    airdash::install();
    airstep::install();
    cancel_in_neutral::install();
    multiple_airdodges::install();
    resets_falses::install();
    wavedash::install();
    //wavestep::install();
}