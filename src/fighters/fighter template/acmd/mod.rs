use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*,*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};
//Write subtitle here (Basically what the new code does)

#[acmd_script( agent = "codenamehere", script = "game_animcmdhere", category = ACMD_GAME )]
unsafe fn codename_animcmd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    //Copy the script for the respective animcmd here, and make changes how u see fit. 
    //You can find it on https://github.com/WuBoytH/SSBU-Dumped-Scripts/tree/main/smashline/13.0.1

}



pub fn install() {
    smashline::install_acmd_scripts!(
        codename_animcmd
    );
}