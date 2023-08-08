pub fn moveset_inputs(fighter: &mut L2CFighterCommon) {
    unsafe {

        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        let cat2 = ControlModule::get_command_flag_cat(fighter.module_accessor, 1);
        let cat3 = ControlModule::get_command_flag_cat(fighter.module_accessor, 2);
        let cat4 = ControlModule::get_command_flag_cat(fighter.module_accessor, 3);

    if (cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK) != 0) 
    }   
}