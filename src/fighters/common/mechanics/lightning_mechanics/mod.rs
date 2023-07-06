

pub mod crimson_cancel;
pub mod lightning_mode;
pub mod ultrainstinct;
pub mod lightning_fsmeter;
pub mod vanish;


pub fn install() {
    //crimson_cancel::install();
    lightning_mode::install();
    //ultrainstinct::install();
    //lightning_fsmeter::install();
    vanish::install();
}