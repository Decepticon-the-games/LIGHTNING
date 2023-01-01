pub mod common;
pub mod mario;
pub mod donkey;
pub mod link;
pub mod samus;
pub mod samusd;
//pub mod yoshi;
//pub mod kirby;
pub mod fox;
pub mod pikachu;



pub mod pichu;
pub mod falco;







pub fn install() {
    common::install();
    mario::install();
    donkey::install();
    link::install();
    samus::install();
    samusd::install();
    //yoshi::install();
    //kirby::install();
    fox::install();
    pikachu::install();
    

    
    pichu::install();
    falco::install();    
    








    
}