

fn exec_escape_air_slide(fighter: L2CFighterCommon) {
    
    /* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
    /* lua2cpp::L2CFighterCommon::exec_escape_air_slide() */

    void __thiscall lua2cpp::L2CFighterCommon::exec_escape_air_slide(L2CFighterCommon *this)

    {
    byte bVar1;
    int iVar2;
    int iVar3;
    SituationKind SVar4;
    ulong uVar5;
    ulong uVar6;
    L2CValue *this_00;
    BattleObjectModuleAccessor *pBVar7;
    BattleObjectModuleAccessor **ppBVar8;
    float fVar9;
    float fVar10;
    float fVar11;
    float fVar12;
    float fVar13;
    L2CValue LStack_110;
    L2CValue LStack_100;
    L2CValue LStack_f0;
    L2CValue LStack_e0;
    L2CValue LStack_d0;
    L2CValue LStack_c0;
    L2CValue LStack_b0;
    L2CValue LStack_a0;
    L2CValue LStack_90;
    L2CValue LStack_80;
    L2CValue LStack_70;
    
    lib::L2CValue::L2CValue(&LStack_70,_FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_STEP);
    iVar2 = lib::L2CValue::as_integer();
    ppBVar8 = (BattleObjectModuleAccessor **)(this + 0x40);
    iVar2 = app::lua_bind::WorkModule__get_int_impl(*ppBVar8,iVar2);
    lib::L2CValue::L2CValue(&LStack_80,iVar2);
    lib::L2CValue::~L2CValue(&LStack_70);
    lib::L2CValue::L2CValue(&LStack_70,FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_BACK_END_FRAME);
    iVar2 = lib::L2CValue::as_integer();
    iVar2 = app::lua_bind::WorkModule__get_int_impl(*ppBVar8,iVar2);
    lib::L2CValue::L2CValue(&LStack_90,iVar2);
    lib::L2CValue::~L2CValue(&LStack_70);
    lib::L2CValue::L2CValue(&LStack_70,0);
    uVar5 = lib::L2CValue::operator==(&LStack_80,(L2CValue *)&LStack_70);
    lib::L2CValue::~L2CValue(&LStack_70);
    if ((uVar5 & 1) != 0) {
        lib::L2CValue::L2CValue(&LStack_a0,_FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
        iVar2 = lib::L2CValue::as_integer();
        iVar2 = app::lua_bind::WorkModule__get_int_impl(*ppBVar8,iVar2);
        lib::L2CValue::L2CValue(&LStack_70,iVar2);
        uVar5 = lib::L2CValue::operator<=(&LStack_90,(L2CValue *)&LStack_70);
        lib::L2CValue::~L2CValue(&LStack_70);
        lib::L2CValue::~L2CValue(&LStack_a0);
        if ((uVar5 & 1) != 0) {
        lib::L2CValue::L2CValue(&LStack_70,0);
        lib::L2CValue::L2CValue(&LStack_a0,_FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
        iVar2 = lib::L2CValue::as_integer();
        iVar3 = lib::L2CValue::as_integer();
        app::lua_bind::WorkModule__set_int_impl(*ppBVar8,iVar2,iVar3);
        lib::L2CValue::~L2CValue(&LStack_a0);
        lib::L2CValue::~L2CValue(&LStack_70);
        lib::L2CValue::L2CValue(&LStack_70,1);
        lib::L2CValue::L2CValue(&LStack_a0,_FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_STEP);
        iVar2 = lib::L2CValue::as_integer();
        iVar3 = lib::L2CValue::as_integer();
        app::lua_bind::WorkModule__set_int_impl(*ppBVar8,iVar2,iVar3);
        lib::L2CValue::~L2CValue(&LStack_a0);
        lib::L2CValue::~L2CValue(&LStack_70);
        lib::L2CValue::L2CValue(&LStack_70,1);
        lib::L2CValue::operator=(&LStack_80,(L2CValue *)&LStack_70);
        lib::L2CValue::~L2CValue(&LStack_70);
        }
    }
    lib::L2CValue::L2CValue(&LStack_b0,FIGHTER_STATUS_ESCAPE_AIR_FLAG_KINE_FALL);
    iVar2 = lib::L2CValue::as_integer();
    bVar1 = app::lua_bind::WorkModule__is_flag_impl(*ppBVar8,iVar2);
    lib::L2CValue::L2CValue(&LStack_a0,(bool)(bVar1 & 1));
    lib::L2CValue::L2CValue(&LStack_70,false);
    uVar5 = lib::L2CValue::operator==(&LStack_a0,(L2CValue *)&LStack_70);
    lib::L2CValue::~L2CValue(&LStack_70);
    lib::L2CValue::~L2CValue(&LStack_a0);
    lib::L2CValue::~L2CValue(&LStack_b0);
    if ((uVar5 & 1) == 0) goto LAB_710015a88c;
    lib::L2CValue::L2CValue(&LStack_70,0);
    uVar5 = lib::L2CValue::operator==(&LStack_80,(L2CValue *)&LStack_70);
    lib::L2CValue::~L2CValue(&LStack_70);
    if ((uVar5 & 1) == 0) {
        lib::L2CValue::L2CValue(&LStack_70,1);
        uVar5 = lib::L2CValue::operator==(&LStack_80,(L2CValue *)&LStack_70);
        lib::L2CValue::~L2CValue(&LStack_70);
        if ((uVar5 & 1) == 0) goto LAB_710015a88c;
        lib::L2CValue::L2CValue(&LStack_70,_FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
        iVar2 = lib::L2CValue::as_integer();
        iVar2 = app::lua_bind::WorkModule__get_int_impl(*ppBVar8,iVar2);
        lib::L2CValue::L2CValue(&LStack_a0,iVar2);
        lib::L2CValue::~L2CValue(&LStack_70);
        lib::L2CValue::L2CValue(&LStack_70,0);
        uVar5 = lib::L2CValue::operator==(&LStack_a0,(L2CValue *)&LStack_70);
        lib::L2CValue::~L2CValue(&LStack_70);
        if ((uVar5 & 1) == 0) {
        lib::L2CValue::L2CValue(&LStack_70,1);
        uVar5 = lib::L2CValue::operator<=(&LStack_70,(L2CValue *)&LStack_a0);
        lib::L2CValue::~L2CValue(&LStack_70);
        if ((uVar5 & 1) != 0) {
            lib::L2CValue::L2CValue
                    (&LStack_c0,_FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_KEEP_AIR_TURNED_OFF);
            iVar2 = lib::L2CValue::as_integer();
            bVar1 = app::lua_bind::WorkModule__is_flag_impl(*ppBVar8,iVar2);
            lib::L2CValue::L2CValue(&LStack_b0,(bool)(bVar1 & 1));
            lib::L2CValue::L2CValue(&LStack_70,false);
            uVar5 = lib::L2CValue::operator==(&LStack_b0,(L2CValue *)&LStack_70);
            lib::L2CValue::~L2CValue(&LStack_70);
            lib::L2CValue::~L2CValue(&LStack_b0);
            lib::L2CValue::~L2CValue(&LStack_c0);
            if ((uVar5 & 1) != 0) {
            lib::L2CValue::L2CValue(&LStack_70,SITUATION_KIND_AIR);
            lib::L2CValue::L2CValue(&LStack_b0,false);
            SVar4 = lib::L2CValue::as_integer();
            bVar1 = lib::L2CValue::as_bool();
            app::lua_bind::StatusModule__set_situation_kind_impl(*ppBVar8,SVar4,(bool)(bVar1 & 1));
            lib::L2CValue::~L2CValue(&LStack_b0);
            lib::L2CValue::~L2CValue(&LStack_70);
            lib::L2CValue::L2CValue
                        (&LStack_70,_FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_KEEP_AIR_TURNED_OFF);
            iVar2 = lib::L2CValue::as_integer();
            app::lua_bind::WorkModule__on_flag_impl(*ppBVar8,iVar2);
            lib::L2CValue::~L2CValue(&LStack_70);
            }
            lib::L2CValue::L2CValue(&LStack_c0,FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
            iVar2 = lib::L2CValue::as_integer();
            bVar1 = app::lua_bind::WorkModule__is_enable_transition_term_impl(*ppBVar8,iVar2);
            lib::L2CValue::L2CValue(&LStack_b0,(bool)(bVar1 & 1));
            lib::L2CValue::L2CValue(&LStack_70,false);
            uVar5 = lib::L2CValue::operator==(&LStack_b0,(L2CValue *)&LStack_70);
            lib::L2CValue::~L2CValue(&LStack_70);
            lib::L2CValue::~L2CValue(&LStack_b0);
            lib::L2CValue::~L2CValue(&LStack_c0);
            if ((uVar5 & 1) != 0) {
            lib::L2CValue::L2CValue(&LStack_70,FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
            iVar2 = lib::L2CValue::as_integer();
            app::lua_bind::WorkModule__enable_transition_term_impl(*ppBVar8,iVar2);
            goto LAB_710015aa44;
            }
        }
        }
        else {
        lib::L2CValue::L2CValue(&LStack_b0,FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
        iVar2 = lib::L2CValue::as_integer();
        fVar9 = (float)app::lua_bind::WorkModule__get_float_impl(*ppBVar8,iVar2);
        lib::L2CValue::L2CValue(&LStack_70,fVar9);
        lib::L2CValue::~L2CValue(&LStack_b0);
        lib::L2CValue::L2CValue(&LStack_c0,_FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
        iVar2 = lib::L2CValue::as_integer();
        fVar9 = (float)app::lua_bind::WorkModule__get_float_impl(*ppBVar8,iVar2);
        lib::L2CValue::L2CValue(&LStack_b0,fVar9);
        lib::L2CValue::~L2CValue(&LStack_c0);
        lib::L2CValue::L2CValue(&LStack_c0,FIGHTER_KINETIC_ENERGY_ID_STOP);
        lib::L2CValue::L2CValue(&LStack_d0,ENERGY_STOP_RESET_TYPE_ESCAPE_AIR_SLIDE);
        lib::L2CValue::L2CValue(&LStack_e0,0.0);
        lib::L2CValue::L2CValue(&LStack_f0,0.0);
        lib::L2CValue::L2CValue(&LStack_100,0.0);
        lib::L2CAgent::clear_lua_stack();
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        app::sv_kinetic_energy::reset_energy(*(lua_State **)(this + 8));
        lib::L2CValue::~L2CValue(&LStack_100);
        lib::L2CValue::~L2CValue(&LStack_f0);
        lib::L2CValue::~L2CValue(&LStack_e0);
        lib::L2CValue::~L2CValue(&LStack_d0);
        lib::L2CValue::~L2CValue(&LStack_c0);
        lib::L2CValue::~L2CValue(&LStack_b0);
    LAB_710015aa44:
        lib::L2CValue::~L2CValue(&LStack_70);
        }
        lib::L2CValue::L2CValue(&LStack_b0,0x15f2c6719b);
        lib::L2CValue::L2CValue(&LStack_c0,0);
        uVar5 = lib::L2CValue::as_integer();
        uVar6 = lib::L2CValue::as_integer();
        fVar9 = (float)app::lua_bind::WorkModule__get_param_float_impl(*ppBVar8,uVar5,uVar6);
        lib::L2CValue::L2CValue(&LStack_70,fVar9);
        uVar5 = lib::L2CValue::operator<=(&LStack_70,(L2CValue *)&LStack_a0);
        lib::L2CValue::~L2CValue(&LStack_70);
        lib::L2CValue::~L2CValue(&LStack_c0);
        lib::L2CValue::~L2CValue(&LStack_b0);
        if ((uVar5 & 1) != 0) {
        lib::L2CValue::L2CValue(&LStack_b0,FIGHTER_KINETIC_ENERGY_ID_STOP);
        lib::L2CAgent::clear_lua_stack();
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        fVar9 = (float)app::sv_kinetic_energy::get_speed_x(*(lua_State **)(this + 8));
        lib::L2CValue::L2CValue(&LStack_70,fVar9);
        lib::L2CValue::~L2CValue(&LStack_b0);
        lib::L2CValue::L2CValue(&LStack_c0,FIGHTER_KINETIC_ENERGY_ID_STOP);
        lib::L2CAgent::clear_lua_stack();
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        fVar9 = (float)app::sv_kinetic_energy::get_speed_y(*(lua_State **)(this + 8));
        lib::L2CValue::L2CValue(&LStack_b0,fVar9);
        lib::L2CValue::~L2CValue(&LStack_c0);
        lib::L2CValue::L2CValue(&LStack_c0,_FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
        iVar2 = lib::L2CValue::as_integer();
        app::lua_bind::WorkModule__on_flag_impl(*ppBVar8,iVar2);
        lib::L2CValue::~L2CValue(&LStack_c0);
        lib::L2CValue::L2CValue(&LStack_c0,FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        lib::L2CValue::L2CValue(&LStack_d0,ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST_NO_CAP);
        lib::L2CValue::L2CValue(&LStack_e0,0.0);
        lib::L2CValue::L2CValue(&LStack_f0,0.0);
        lib::L2CValue::L2CValue(&LStack_100,0.0);
        lib::L2CValue::L2CValue(&LStack_110,0.0);
        lib::L2CAgent::clear_lua_stack();
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        app::sv_kinetic_energy::reset_energy(*(lua_State **)(this + 8));
        lib::L2CValue::~L2CValue(&LStack_110);
        lib::L2CValue::~L2CValue(&LStack_100);
        lib::L2CValue::~L2CValue(&LStack_f0);
        lib::L2CValue::~L2CValue(&LStack_e0);
        lib::L2CValue::~L2CValue(&LStack_d0);
        lib::L2CValue::~L2CValue(&LStack_c0);
        lib::L2CValue::L2CValue(&LStack_c0,FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        lib::L2CAgent::clear_lua_stack();
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        app::sv_kinetic_energy::enable(*(lua_State **)(this + 8));
        lib::L2CValue::~L2CValue(&LStack_c0);
        lib::L2CValue::L2CValue(&LStack_c0,FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        lib::L2CValue::L2CValue(&LStack_d0,ENERGY_GRAVITY_RESET_TYPE_GRAVITY);
        lib::L2CValue::L2CValue(&LStack_e0,0.0);
        lib::L2CValue::L2CValue(&LStack_f0,0.0);
        lib::L2CValue::L2CValue(&LStack_100,0.0);
        lib::L2CValue::L2CValue(&LStack_110,0.0);
        lib::L2CAgent::clear_lua_stack();
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        app::sv_kinetic_energy::reset_energy(*(lua_State **)(this + 8));
        lib::L2CValue::~L2CValue(&LStack_110);
        lib::L2CValue::~L2CValue(&LStack_100);
        lib::L2CValue::~L2CValue(&LStack_f0);
        lib::L2CValue::~L2CValue(&LStack_e0);
        lib::L2CValue::~L2CValue(&LStack_d0);
        lib::L2CValue::~L2CValue(&LStack_c0);
        lib::L2CValue::L2CValue(&LStack_c0,FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        lib::L2CAgent::clear_lua_stack();
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        app::sv_kinetic_energy::enable(*(lua_State **)(this + 8));
        lib::L2CValue::~L2CValue(&LStack_c0);
        lib::L2CValue::L2CValue(&LStack_c0,FIGHTER_KINETIC_ENERGY_ID_STOP);
        lib::L2CValue::operator[]((int)this + 200);
        iVar2 = lib::L2CValue::as_integer();
        pBVar7 = (BattleObjectModuleAccessor *)lib::L2CValue::as_pointer();
        app::KineticUtility::clear_unable_energy(iVar2,pBVar7);
        lib::L2CValue::~L2CValue(&LStack_c0);
        lib::L2CValue::L2CValue(&LStack_c0,FIGHTER_STATUS_ESCAPE_AIR_FLAG_KINE_FALL);
        iVar2 = lib::L2CValue::as_integer();
        app::lua_bind::WorkModule__on_flag_impl(*ppBVar8,iVar2);
        lib::L2CValue::~L2CValue(&LStack_c0);
        lib::L2CValue::L2CValue(&LStack_c0,FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
        iVar2 = lib::L2CValue::as_integer();
        app::lua_bind::WorkModule__off_flag_impl(*ppBVar8,iVar2);
        lib::L2CValue::~L2CValue(&LStack_c0);
        lib::L2CValue::L2CValue(&LStack_c0,0);
        lib::L2CValue::L2CValue(&LStack_d0,_FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
        iVar2 = lib::L2CValue::as_integer();
        iVar3 = lib::L2CValue::as_integer();
        app::lua_bind::WorkModule__set_int_impl(*ppBVar8,iVar2,iVar3);
        lib::L2CValue::~L2CValue(&LStack_d0);
        lib::L2CValue::~L2CValue(&LStack_c0);
        lib::L2CValue::L2CValue(&LStack_c0,2);
        lib::L2CValue::L2CValue(&LStack_d0,_FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_STEP);
        iVar2 = lib::L2CValue::as_integer();
        iVar3 = lib::L2CValue::as_integer();
        app::lua_bind::WorkModule__set_int_impl(*ppBVar8,iVar2,iVar3);
        lib::L2CValue::~L2CValue(&LStack_d0);
        lib::L2CValue::~L2CValue(&LStack_c0);
        lib::L2CValue::~L2CValue(&LStack_b0);
        this_00 = &LStack_70;
        goto LAB_710015a6c4;
        }
    }
    else {
        lib::L2CValue::L2CValue(&LStack_c0,_FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_INT_SLIDE_FRAME);
        iVar2 = lib::L2CValue::as_integer();
        iVar2 = app::lua_bind::WorkModule__get_int_impl(*ppBVar8,iVar2);
        lib::L2CValue::L2CValue(&LStack_b0,iVar2);
        lib::L2CValue::L2CValue(&LStack_70,1.0);
        lib::L2CValue::operator-(&LStack_90,(L2CValue *)&LStack_70);
        lib::L2CValue::~L2CValue(&LStack_70);
        lib::L2CValue::operator/(&LStack_b0,(L2CValue *)&LStack_d0);
        lib::L2CValue::~L2CValue(&LStack_d0);
        lib::L2CValue::~L2CValue(&LStack_b0);
        lib::L2CValue::~L2CValue(&LStack_c0);
        lib::L2CValue::L2CValue(&LStack_70,_FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_BACK_ACCEL);
        iVar2 = lib::L2CValue::as_integer();
        fVar9 = (float)app::lua_bind::WorkModule__get_float_impl(*ppBVar8,iVar2);
        lib::L2CValue::L2CValue(&LStack_b0,fVar9);
        lib::L2CValue::~L2CValue(&LStack_70);
        lib::L2CValue::L2CValue(&LStack_70,0.5);
        lib::L2CValue::L2CValue(&LStack_d0,0.8);
        lib::L2CValue::L2CValue(&LStack_e0,0.9);
        lib::L2CValue::L2CValue(&LStack_f0,1.0);
        fVar9 = (float)lib::L2CValue::as_number();
        fVar10 = (float)lib::L2CValue::as_number();
        fVar11 = (float)lib::L2CValue::as_number();
        fVar12 = (float)lib::L2CValue::as_number();
        fVar13 = (float)lib::L2CValue::as_number();
        fVar9 = (float)app::sv_math::bezier_curve(fVar9,fVar10,fVar11,fVar12,fVar13);
        lib::L2CValue::L2CValue(&LStack_c0,fVar9);
        lib::L2CValue::~L2CValue(&LStack_f0);
        lib::L2CValue::~L2CValue(&LStack_e0);
        lib::L2CValue::~L2CValue(&LStack_d0);
        lib::L2CValue::~L2CValue(&LStack_70);
        lib::L2CValue::operator-(&LStack_c0,(L2CValue *)&LStack_b0);
        lib::L2CValue::L2CValue(&LStack_100,0xcad2ee25e);
        lib::L2CValue::L2CValue(&LStack_110,0x1eb3ae94e3);
        uVar5 = lib::L2CValue::as_integer();
        uVar6 = lib::L2CValue::as_integer();
        fVar9 = (float)app::lua_bind::WorkModule__get_param_float_impl(*ppBVar8,uVar5,uVar6);
        lib::L2CValue::L2CValue(&LStack_f0,fVar9);
        lib::L2CValue::operator-(&LStack_f0);
        lib::L2CValue::operator*(&LStack_70,(L2CValue *)&LStack_e0);
        lib::L2CValue::~L2CValue(&LStack_e0);
        lib::L2CValue::~L2CValue(&LStack_f0);
        lib::L2CValue::~L2CValue(&LStack_110);
        lib::L2CValue::~L2CValue(&LStack_100);
        lib::L2CValue::~L2CValue(&LStack_70);
        lib::L2CValue::L2CValue(&LStack_70,0.0);
        lib::L2CValue::operator+(&LStack_c0,(L2CValue *)&LStack_70);
        lib::L2CValue::~L2CValue(&LStack_70);
        lib::L2CValue::L2CValue(&LStack_70,_FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_BACK_ACCEL);
        fVar9 = (float)lib::L2CValue::as_number();
        iVar2 = lib::L2CValue::as_integer();
        app::lua_bind::WorkModule__set_float_impl(*ppBVar8,fVar9,iVar2);
        lib::L2CValue::~L2CValue(&LStack_70);
        lib::L2CValue::~L2CValue(&LStack_e0);
        lib::L2CValue::L2CValue(&LStack_f0,FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
        iVar2 = lib::L2CValue::as_integer();
        fVar9 = (float)app::lua_bind::WorkModule__get_float_impl(*ppBVar8,iVar2);
        lib::L2CValue::L2CValue(&LStack_e0,fVar9);
        lib::L2CValue::operator*(&LStack_d0,(L2CValue *)&LStack_e0);
        lib::L2CValue::~L2CValue(&LStack_e0);
        lib::L2CValue::~L2CValue(&LStack_f0);
        lib::L2CValue::L2CValue(&LStack_100,_FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
        iVar2 = lib::L2CValue::as_integer();
        fVar9 = (float)app::lua_bind::WorkModule__get_float_impl(*ppBVar8,iVar2);
        lib::L2CValue::L2CValue(&LStack_f0,fVar9);
        lib::L2CValue::operator*(&LStack_d0,(L2CValue *)&LStack_f0);
        lib::L2CValue::~L2CValue(&LStack_f0);
        lib::L2CValue::~L2CValue(&LStack_100);
        lib::L2CValue::L2CValue(&LStack_f0,FIGHTER_KINETIC_ENERGY_ID_STOP);
        lib::L2CAgent::clear_lua_stack();
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        lib::L2CAgent::push_lua_stack((L2CValue *)this);
        app::sv_kinetic_energy::set_speed(*(lua_State **)(this + 8));
        lib::L2CValue::~L2CValue(&LStack_f0);
        lib::L2CValue::~L2CValue(&LStack_e0);
        lib::L2CValue::~L2CValue(&LStack_70);
        lib::L2CValue::~L2CValue(&LStack_d0);
        lib::L2CValue::~L2CValue(&LStack_c0);
        this_00 = &LStack_b0;
    LAB_710015a6c4:
        lib::L2CValue::~L2CValue(this_00);
    }
    lib::L2CValue::~L2CValue(&LStack_a0);
    LAB_710015a88c:
    lib::L2CValue::~L2CValue(&LStack_90);
    lib::L2CValue::~L2CValue(&LStack_80);
    return;
    }


}

//


/* lua2cpp::L2CFighterCommon::sub_transition_group_check_air_escape() */

void __thiscall
lua2cpp::L2CFighterCommon::sub_transition_group_check_air_escape(L2CFighterCommon *this){
  bool bVar1;
  byte bVar2;
  int iVar3;
  int iVar4;
  L2CValue *pLVar5;
  code *pcVar6;
  L2CValue *pLVar7;
  ulong uVar8;
  L2CValue *in_x8;
  L2CValue LStack_90;
  L2CValue LStack_80;
  L2CValue LStack_70;
  L2CValue LStack_60;
  L2CValue LStack_50;
  
  iVar3 = (L2CValue)this + 200;
  pLVar5 = (L2CValue *)lib::L2CValue::operator[](iVar3);
  lib::L2CValue::L2CValue(&LStack_60,pLVar5);
  bVar1 = lib::L2CValue::operator.cast.to.bool(&LStack_60);
  if (bVar1) {
    pcVar6 = (code *)lib::L2CValue::as_pointer();
    (*pcVar6)(this);
  }
  else {
    lib::L2CValue::L2CValue(&LStack_50,false);
  }
  bVar1 = lib::L2CValue::operator.cast.to.bool(&LStack_50);
  lib::L2CValue::~L2CValue(&LStack_50);
  lib::L2CValue::~L2CValue(&LStack_60);
  if (bVar1) {
  LAB_7100049970:
    bVar1 = true;
  }
  else {
    pLVar7 = (L2CValue *)lib::L2CValue::operator[](iVar3);
    lib::L2CValue::L2CValue(&LStack_50,SITUATION_KIND_AIR);
    uVar8 = lib::L2CValue::operator==(pLVar7,(L2CValue *)&LStack_50);
    lib::L2CValue::~L2CValue(&LStack_50);
    if ((uVar8 & 1) != 0) {
      lib::L2CValue::L2CValue(&LStack_70,FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
      iVar4 = lib::L2CValue::as_integer();
      bVar2 = app::lua_bind::WorkModule__is_flag_impl
                        (*(BattleObjectModuleAccessor **)(this + 0x40),iVar4);
      lib::L2CValue::L2CValue(&LStack_50,(bool)(bVar2 & 1));
      bVar1 = lib::L2CValue::operator.cast.to.bool(&LStack_50);
      lib::L2CValue::~L2CValue(&LStack_50);
      lib::L2CValue::~L2CValue(&LStack_70);
      if (!bVar1) {
        pLVar7 = (L2CValue *)lib::L2CValue::operator[](iVar3);
        lib::L2CValue::L2CValue(&LStack_50,FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE);
        lib::L2CValue::operator&(pLVar7,(L2CValue *)&LStack_50);
        lib::L2CValue::~L2CValue(&LStack_50);
        bVar1 = lib::L2CValue::operator.cast.to.bool(&LStack_70);
        lib::L2CValue::~L2CValue(&LStack_70);
        if (bVar1) {
          lib::L2CValue::L2CValue(&LStack_70,FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
          iVar3 = lib::L2CValue::as_integer();
          bVar2 = app::lua_bind::WorkModule__is_enable_transition_term_impl
                            (*(BattleObjectModuleAccessor **)(this + 0x40),iVar3);
          lib::L2CValue::L2CValue(&LStack_50,(bool)(bVar2 & 1));
          bVar1 = lib::L2CValue::operator.cast.to.bool(&LStack_50);
          lib::L2CValue::~L2CValue(&LStack_50);
          lib::L2CValue::~L2CValue(&LStack_70);
          if (bVar1) {
            lib::L2CValue::L2CValue(&LStack_80,FIGHTER_STATUS_KIND_ESCAPE_AIR);
            lib::L2CValue::L2CValue(&LStack_90,true);
            L2CFighterBase::change_status((L2CValue)this,(L2CValue)&LStack_80);
            lib::L2CValue::~L2CValue(&LStack_90);
            lib::L2CValue::~L2CValue(&LStack_80);
            goto LAB_7100049970;
          }
        }
      }
    }
    bVar1 = false;
  }
  lib::L2CValue::L2CValue(in_x8,bVar1);
  return;
}

//


/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* lua2cpp::L2CFighterCommon::sub_escape_uniq_process_common_initStatus_common(lib::L2CValue,
   lib::L2CValue) */

void lua2cpp::L2CFighterCommon::sub_escape_uniq_process_common_initStatus_common
(L2CValue param_1,L2CValue param_2){
  byte bVar1;
  bool bVar2;
  int iVar3;
  int iVar4;
  L2CValue LVar5;
  HitStatus HVar6;
  int iVar7;
  L2CValue *pLVar8;
  L2CValue *pLVar9;
  ulong uVar10;
  L2CValue *pLVar11;
  ulong uVar12;
  Hash40 HVar13;
  BattleObjectModuleAccessor **ppBVar14;
  float fVar15;
  float fVar16;
  L2CValue LStack_1c0;
  L2CValue LStack_1b0;
  L2CValue LStack_1a0;
  L2CValue LStack_190;
  L2CValue LStack_180;
  L2CValue LStack_170;
  L2CValue LStack_160;
  L2CValue LStack_150;
  L2CValue LStack_140;
  L2CValue LStack_130;
  L2CValue LStack_120;
  L2CValue LStack_110;
  L2CValue LStack_100;
  L2CValue LStack_f0;
  L2CValue LStack_e0;
  L2CValue LStack_d0;
  L2CValue LStack_c0;
  L2CValue LStack_b0;
  L2CValue LStack_a0;
  L2CValue LStack_90;
  L2CValue LStack_80;
  L2CValue LStack_70;
  L2CValue LStack_60;

  pLVar8 = (L2CValue *)(ulong)param_1;
  lib::L2CValue::L2CValue(&LStack_70,0);
  iVar3 = param_1 + 200;
  pLVar9 = (L2CValue *)lib::L2CValue::operator[](iVar3);
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_STATUS_KIND_ESCAPE_AIR);
  uVar10 = lib::L2CValue::operator==(pLVar9,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  if ((uVar10 & 1) == 0) {
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
  lib::L2CAgent::clear_lua_stack();
  lib::L2CAgent::push_lua_stack(pLVar8);
  app::sv_kinetic_energy::clear_speed_ex(*(lua_State **)(pLVar8 + 2));
  }
  else {
  pLVar11 = (L2CValue *)lib::L2CValue::operator[](iVar3);
  lib::L2CValue::operator=(&LStack_70,pLVar11);
  fVar15 = (float)lib::L2CValue::as_number();
  fVar16 = (float)lib::L2CValue::as_number();
  fVar15 = (float)app::sv_math::vec2_length(fVar15,fVar16);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::L2CValue(&LStack_90,0x6e5ec7051);
  lib::L2CValue::L2CValue(&LStack_a0,0x162e4d87e8);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  ppBVar14 = (BattleObjectModuleAccessor **)(pLVar8 + 0x10);
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl(*ppBVar14,uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_80,fVar15);
  uVar10 = lib::L2CValue::operator<=(&LStack_80,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_80);
  lib::L2CValue::~L2CValue(&LStack_a0);
  lib::L2CValue::~L2CValue(&LStack_90);
  lib::L2CValue::~L2CValue(&LStack_60);
  if ((uVar10 & 1) != 0) {
  lib::L2CValue::L2CValue(&LStack_60,_FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE);
  iVar4 = lib::L2CValue::as_integer();
  app::lua_bind::WorkModule__on_flag_impl(*ppBVar14,iVar4);
  lib::L2CValue::~L2CValue(&LStack_60);
  }
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_STATUS_KIND_DAMAGE_FALL);
  uVar10 = lib::L2CValue::operator==(&LStack_70,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  if ((uVar10 & 1) == 0) {
  lib::L2CValue::L2CValue(&LStack_60,_FIGHTER_STATUS_KIND_TREAD_FALL);
  uVar10 = lib::L2CValue::operator==(&LStack_70,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  if ((uVar10 & 1) != 0) goto LAB_710015c0f0;
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_STATUS_KIND_DAMAGE_FLY);
  uVar10 = lib::L2CValue::operator==(&LStack_70,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  if ((uVar10 & 1) != 0) {
  LAB_710015c258:
  lib::L2CValue::L2CValue
        (&LStack_60,_FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND);
  iVar4 = lib::L2CValue::as_integer();
  app::lua_bind::WorkModule__on_flag_impl(*ppBVar14,iVar4);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_AIR);
  iVar4 = lib::L2CValue::as_integer();
  app::lua_bind::WorkModule__on_flag_impl(*ppBVar14,iVar4);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::L2CValue(&LStack_60,0.0);
  lib::L2CAgent::clear_lua_stack();
  lib::L2CAgent::push_lua_stack(pLVar8);
  app::sv_kinetic_energy::controller_set_accel_x_mul(*(lua_State **)(pLVar8 + 2));
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::L2CValue(&LStack_60,0.0);
  lib::L2CAgent::clear_lua_stack();
  lib::L2CAgent::push_lua_stack(pLVar8);
  app::sv_kinetic_energy::controller_set_accel_x_add(*(lua_State **)(pLVar8 + 2));
  goto LAB_710015c300;
  }
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL);
  uVar10 = lib::L2CValue::operator==(&LStack_70,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  if ((uVar10 & 1) != 0) goto LAB_710015c258;
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR);
  uVar10 = lib::L2CValue::operator==(&LStack_70,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  if ((uVar10 & 1) != 0) goto LAB_710015c258;
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR);
  uVar10 = lib::L2CValue::operator==(&LStack_70,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  if ((uVar10 & 1) != 0) goto LAB_710015c258;
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U);
  uVar10 = lib::L2CValue::operator==(&LStack_70,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  if ((uVar10 & 1) != 0) goto LAB_710015c258;
  lib::L2CValue::L2CValue(&LStack_60,_FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D);
  uVar10 = lib::L2CValue::operator==(&LStack_70,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  if ((uVar10 & 1) != 0) goto LAB_710015c258;
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY);
  uVar10 = lib::L2CValue::operator==(&LStack_70,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  if ((uVar10 & 1) != 0) goto LAB_710015c258;
  }
  else {
  LAB_710015c0f0:
  lib::L2CValue::L2CValue(&LStack_60,_FIGHTER_STATUS_ESCAPE_AIR_FLAG_PREV_STATUS_PASSIVE_GROUND)
  ;
  iVar4 = lib::L2CValue::as_integer();
  app::lua_bind::WorkModule__on_flag_impl(*ppBVar14,iVar4);
  LAB_710015c300:
  lib::L2CValue::~L2CValue(&LStack_60);
  }
  lib::L2CValue::L2CValue(&LStack_90,_FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE);
  iVar4 = lib::L2CValue::as_integer();
  bVar1 = app::lua_bind::WorkModule__is_flag_impl(*ppBVar14,iVar4);
  lib::L2CValue::L2CValue(&LStack_80,(bool)(bVar1 & 1));
  lib::L2CValue::L2CValue(&LStack_60,false);
  uVar10 = lib::L2CValue::operator==(&LStack_80,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_80);
  lib::L2CValue::~L2CValue(&LStack_90);
  if ((uVar10 & 1) == 0) {
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
  iVar4 = lib::L2CValue::as_integer();
  app::lua_bind::WorkModule__unable_transition_term_impl(*ppBVar14,iVar4);
  LAB_710015c998:
  lib::L2CValue::~L2CValue(&LStack_60);
  }
  else {
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
  iVar4 = lib::L2CValue::as_integer();
  app::lua_bind::WorkModule__enable_transition_term_impl(*ppBVar14,iVar4);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::L2CValue(&LStack_a0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_b0,0x1cce73b80e);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl(*ppBVar14,uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_90,fVar15);
  lib::L2CValue::L2CValue(&LStack_60,0.0);
  lib::L2CValue::operator+(&LStack_90,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_STATUS_ESCAPE_AIR_STIFF_START_FRAME);
  fVar15 = (float)lib::L2CValue::as_number();
  iVar4 = lib::L2CValue::as_integer();
  app::lua_bind::WorkModule__set_float_impl(*ppBVar14,fVar15,iVar4);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_80);
  lib::L2CValue::~L2CValue(&LStack_90);
  lib::L2CValue::~L2CValue(&LStack_b0);
  lib::L2CValue::~L2CValue(&LStack_a0);
  lib::L2CValue::L2CValue(&LStack_a0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_b0,0x1693ba4d0b);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl(*ppBVar14,uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_90,fVar15);
  lib::L2CValue::L2CValue(&LStack_60,0.0);
  lib::L2CValue::operator+(&LStack_90,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::L2CValue(&LStack_60,_FIGHTER_STATUS_ESCAPE_AIR_STIFF_FRAME);
  fVar15 = (float)lib::L2CValue::as_number();
  iVar4 = lib::L2CValue::as_integer();
  app::lua_bind::WorkModule__set_float_impl(*ppBVar14,fVar15,iVar4);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_80);
  lib::L2CValue::~L2CValue(&LStack_90);
  lib::L2CValue::~L2CValue(&LStack_b0);
  lib::L2CValue::~L2CValue(&LStack_a0);
  lib::L2CValue::L2CValue(&LStack_80,0x2ea659cf56);
  lib::L2CValue::L2CValue(&LStack_90,0);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  iVar4 = app::lua_bind::WorkModule__get_param_int_impl(*ppBVar14,uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,iVar4);
  bVar2 = lib::L2CValue::operator.cast.to.bool(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_90);
  lib::L2CValue::~L2CValue(&LStack_80);
  if (bVar2) {
  HVar13 = app::lua_bind::MotionModule__motion_kind_impl(*ppBVar14);
  lib::L2CValue::L2CValue(&LStack_80,HVar13);
  lib::L2CValue::L2CValue(&LStack_60,0xd0b71815b);
  uVar10 = lib::L2CValue::operator==(&LStack_80,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  if ((uVar10 & 1) == 0) {
  HVar13 = app::lua_bind::MotionModule__motion_kind_impl(*ppBVar14);
  lib::L2CValue::L2CValue(&LStack_90,HVar13);
  lib::L2CValue::L2CValue(&LStack_60,0xd0c1c4542);
  uVar10 = lib::L2CValue::operator==(&LStack_90,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_90);
  lib::L2CValue::~L2CValue(&LStack_80);
  if ((uVar10 & 1) == 0) goto LAB_710015c9a0;
  }
  else {
  lib::L2CValue::~L2CValue(&LStack_80);
  }
  lib::L2CValue::L2CValue(&LStack_90,0x271984ee09);
  lib::L2CValue::L2CValue(&LStack_a0,0);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl(*ppBVar14,uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_80,fVar15);
  lib::L2CValue::L2CValue(&LStack_d0,0x2bf4bef265);
  lib::L2CValue::L2CValue(&LStack_e0,0);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl(*ppBVar14,uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_c0,fVar15);
  fVar15 = (float)app::lua_bind::MotionModule__frame_impl(*ppBVar14);
  lib::L2CValue::L2CValue(&LStack_f0,fVar15);
  lib::L2CValue::operator*(&LStack_c0,(L2CValue *)&LStack_f0);
  lib::L2CValue::operator-(&LStack_80,(L2CValue *)&LStack_b0);
  lib::L2CValue::~L2CValue(&LStack_b0);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::~L2CValue(&LStack_c0);
  lib::L2CValue::~L2CValue(&LStack_e0);
  lib::L2CValue::~L2CValue(&LStack_d0);
  lib::L2CValue::~L2CValue(&LStack_80);
  lib::L2CValue::~L2CValue(&LStack_a0);
  lib::L2CValue::~L2CValue(&LStack_90);
  lib::L2CValue::L2CValue(&LStack_a0,0x12ec5626fe);
  lib::L2CValue::L2CValue(&LStack_b0,0);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl(*ppBVar14,uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_90,fVar15);
  lib::L2CValue::operator-(&LStack_90);
  uVar10 = lib::L2CValue::operator<(&LStack_60,(L2CValue *)&LStack_80);
  lib::L2CValue::~L2CValue(&LStack_80);
  lib::L2CValue::~L2CValue(&LStack_90);
  lib::L2CValue::~L2CValue(&LStack_b0);
  lib::L2CValue::~L2CValue(&LStack_a0);
  if ((uVar10 & 1) != 0) {
  lib::L2CValue::L2CValue(&LStack_a0,0x12ec5626fe);
  lib::L2CValue::L2CValue(&LStack_b0,0);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl(*ppBVar14,uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_90,fVar15);
  lib::L2CValue::operator-(&LStack_90);
  lib::L2CValue::operator=(&LStack_60,(L2CValue *)&LStack_80);
  lib::L2CValue::~L2CValue(&LStack_80);
  lib::L2CValue::~L2CValue(&LStack_90);
  lib::L2CValue::~L2CValue(&LStack_b0);
  lib::L2CValue::~L2CValue(&LStack_a0);
  }
  lib::L2CValue::L2CValue(&LStack_80,FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
  lib::L2CValue::L2CValue(&LStack_90,ENERGY_GRAVITY_RESET_TYPE_GRAVITY);
  lib::L2CValue::L2CValue(&LStack_a0,0.0);
  lib::L2CValue::L2CValue(&LStack_b0,0.0);
  lib::L2CValue::L2CValue(&LStack_c0,0.0);
  lib::L2CValue::L2CValue(&LStack_d0,0.0);
  lib::L2CAgent::clear_lua_stack();
  lib::L2CAgent::push_lua_stack(pLVar8);
  lib::L2CAgent::push_lua_stack(pLVar8);
  lib::L2CAgent::push_lua_stack(pLVar8);
  lib::L2CAgent::push_lua_stack(pLVar8);
  lib::L2CAgent::push_lua_stack(pLVar8);
  lib::L2CAgent::push_lua_stack(pLVar8);
  lib::L2CAgent::push_lua_stack(pLVar8);
  app::sv_kinetic_energy::reset_energy(*(lua_State **)(pLVar8 + 2));
  lib::L2CValue::~L2CValue(&LStack_d0);
  lib::L2CValue::~L2CValue(&LStack_c0);
  lib::L2CValue::~L2CValue(&LStack_b0);
  lib::L2CValue::~L2CValue(&LStack_a0);
  lib::L2CValue::~L2CValue(&LStack_90);
  lib::L2CValue::~L2CValue(&LStack_80);
  lib::L2CValue::L2CValue(&LStack_80,FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
  lib::L2CAgent::clear_lua_stack();
  lib::L2CAgent::push_lua_stack(pLVar8);
  app::sv_kinetic_energy::enable(*(lua_State **)(pLVar8 + 2));
  lib::L2CValue::~L2CValue(&LStack_80);
  goto LAB_710015c998;
  }
  }
  LAB_710015c9a0:
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
  iVar4 = lib::L2CValue::as_integer();
  app::lua_bind::WorkModule__on_flag_impl(*ppBVar14,iVar4);
  }
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::L2CValue(&LStack_80,0);
  lib::L2CValue::L2CValue(&LStack_90,1.0);
  lib::L2CValue::L2CValue(&LStack_a0,0.0);
  lib::L2CValue::L2CValue(&LStack_b0,0.0);
  lib::L2CValue::L2CValue(&LStack_c0,0.0);
  lib::L2CValue::L2CValue(&LStack_d0,0.0);
  lib::L2CValue::L2CValue(&LStack_e0,0.0);
  pLVar9 = (L2CValue *)lib::L2CValue::operator[](iVar3);
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_STATUS_KIND_ESCAPE);
  uVar10 = lib::L2CValue::operator==(pLVar9,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  if ((uVar10 & 1) == 0) {
  pLVar9 = (L2CValue *)lib::L2CValue::operator[](iVar3);
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_STATUS_KIND_ESCAPE_F);
  uVar10 = lib::L2CValue::operator==(pLVar9,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  if ((uVar10 & 1) == 0) {
  pLVar9 = (L2CValue *)lib::L2CValue::operator[](iVar3);
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_STATUS_KIND_ESCAPE_B);
  uVar10 = lib::L2CValue::operator==(pLVar9,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  if ((uVar10 & 1) == 0) {
  pLVar9 = (L2CValue *)lib::L2CValue::operator[](iVar3);
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_STATUS_KIND_ESCAPE_AIR);
  uVar10 = lib::L2CValue::operator==(pLVar9,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  if ((uVar10 & 1) == 0) goto LAB_710015d4dc;
  lib::L2CValue::L2CValue(&LStack_f0,_FIGHTER_INSTANCE_WORK_ID_FLOAT_USED_ESCAPE_AIR);
  iVar4 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_float_impl
                        (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),iVar4);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_80,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0x6e5ec7051);
  lib::L2CValue::L2CValue(&LStack_100,0x1e0e0e80a0);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                        (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_a0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_100,_FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE);
  iVar4 = lib::L2CValue::as_integer();
  bVar1 = app::lua_bind::WorkModule__is_flag_impl
                (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),iVar4);
  lib::L2CValue::L2CValue(&LStack_f0,(bool)(bVar1 & 1));
  lib::L2CValue::L2CValue(&LStack_60,false);
  uVar10 = lib::L2CValue::operator==(&LStack_f0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::~L2CValue(&LStack_100);
  if ((uVar10 & 1) == 0) {
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x1e411ee639);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                          (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_b0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x2610eab14b);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                          (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_c0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x21a3d8443f);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                          (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_d0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x29bb8705c6);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                          (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_e0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0x1060d3285b);
  HVar13 = lib::L2CValue::as_hash();
  bVar1 = app::lua_bind::MotionModule__is_flag_start_1_frame_from_motion_kind_impl
                  (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),HVar13);
  lib::L2CValue::L2CValue(&LStack_60,(bool)(bVar1 & 1));
  bVar2 = lib::L2CValue::operator.cast.to.bool(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_f0);
  if (!bVar2) goto LAB_710015d4dc;
  lib::L2CValue::L2CValue(&LStack_60,1);
  lib::L2CValue::operator-(&LStack_b0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::operator=(&LStack_b0,(L2CValue *)&LStack_f0);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_60,1);
  lib::L2CValue::operator-(&LStack_c0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::operator=(&LStack_c0,(L2CValue *)&LStack_f0);
  }
  else {
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x1848215e08);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                          (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_b0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x20b22b3466);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                          (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_c0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x1b1c0bc21c);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                          (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_d0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x2321854eff);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                          (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_e0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0xae47033ba);
  HVar13 = lib::L2CValue::as_hash();
  bVar1 = app::lua_bind::MotionModule__is_flag_start_1_frame_from_motion_kind_impl
                  (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),HVar13);
  lib::L2CValue::L2CValue(&LStack_60,(bool)(bVar1 & 1));
  bVar2 = lib::L2CValue::operator.cast.to.bool(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_f0);
  if (!bVar2) goto LAB_710015d4dc;
  lib::L2CValue::L2CValue(&LStack_60,1);
  lib::L2CValue::operator-(&LStack_b0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::operator=(&LStack_b0,(L2CValue *)&LStack_f0);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_60,1);
  lib::L2CValue::operator-(&LStack_c0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::operator=(&LStack_c0,(L2CValue *)&LStack_f0);
  }
  }
  else {
  lib::L2CValue::L2CValue(&LStack_f0,FIGHTER_INSTANCE_WORK_ID_FLOAT_USED_ESCAPE_B);
  iVar4 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_float_impl
                        (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),iVar4);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_80,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0x6e5ec7051);
  lib::L2CValue::L2CValue(&LStack_100,0x1c199b4740);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                        (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_a0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x1646545a65);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                        (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_b0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x1ee955870c);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                        (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_c0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x191807dad6);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                        (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_d0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x211c90a112);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                        (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_e0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0x8b12770b1);
  HVar13 = lib::L2CValue::as_hash();
  bVar1 = app::lua_bind::MotionModule__is_flag_start_1_frame_from_motion_kind_impl
                (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),HVar13);
  lib::L2CValue::L2CValue(&LStack_60,(bool)(bVar1 & 1));
  bVar2 = lib::L2CValue::operator.cast.to.bool(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_f0);
  if (!bVar2) goto LAB_710015d4dc;
  lib::L2CValue::L2CValue(&LStack_60,1);
  lib::L2CValue::operator-(&LStack_b0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::operator=(&LStack_b0,(L2CValue *)&LStack_f0);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_60,1);
  lib::L2CValue::operator-(&LStack_c0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::operator=(&LStack_c0,(L2CValue *)&LStack_f0);
  }
  }
  else {
  lib::L2CValue::L2CValue(&LStack_f0,_FIGHTER_INSTANCE_WORK_ID_FLOAT_USED_ESCAPE_F);
  iVar4 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_float_impl
                      (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),iVar4);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_80,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0x6e5ec7051);
  lib::L2CValue::L2CValue(&LStack_100,0x1c0b6243fb);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                      (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_a0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x16cc283f06);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                      (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_b0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x1ee909269f);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                      (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_c0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x1913af0dd0);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                      (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_d0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x21360437cc);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                      (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_e0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0x8b64ab4a8);
  HVar13 = lib::L2CValue::as_hash();
  bVar1 = app::lua_bind::MotionModule__is_flag_start_1_frame_from_motion_kind_impl
              (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),HVar13);
  lib::L2CValue::L2CValue(&LStack_60,(bool)(bVar1 & 1));
  bVar2 = lib::L2CValue::operator.cast.to.bool(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_f0);
  if (!bVar2) goto LAB_710015d4dc;
  lib::L2CValue::L2CValue(&LStack_60,1);
  lib::L2CValue::operator-(&LStack_b0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::operator=(&LStack_b0,(L2CValue *)&LStack_f0);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_60,1);
  lib::L2CValue::operator-(&LStack_c0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::operator=(&LStack_c0,(L2CValue *)&LStack_f0);
  }
  }
  else {
  lib::L2CValue::L2CValue(&LStack_f0,FIGHTER_INSTANCE_WORK_ID_FLOAT_USED_ESCAPE);
  iVar4 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_float_impl
                    (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),iVar4);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_80,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0x6e5ec7051);
  lib::L2CValue::L2CValue(&LStack_100,0x1a746bbfe4);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                    (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_a0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x1603a1f381);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                    (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_b0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x1ee9b065b9);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                    (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_c0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x1904fea3dc);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                    (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_d0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0xcad2ee25e);
  lib::L2CValue::L2CValue(&LStack_100,0x21632d1a70);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                    (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,fVar15);
  lib::L2CValue::operator=(&LStack_e0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_f0,0x625e8b08e);
  HVar13 = lib::L2CValue::as_hash();
  bVar1 = app::lua_bind::MotionModule__is_flag_start_1_frame_from_motion_kind_impl
            (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),HVar13);
  lib::L2CValue::L2CValue(&LStack_60,(bool)(bVar1 & 1));
  bVar2 = lib::L2CValue::operator.cast.to.bool(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_f0);
  if (!bVar2) goto LAB_710015d4dc;
  lib::L2CValue::L2CValue(&LStack_60,1);
  lib::L2CValue::operator-(&LStack_b0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::operator=(&LStack_b0,(L2CValue *)&LStack_f0);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::L2CValue(&LStack_60,1);
  lib::L2CValue::operator-(&LStack_c0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::operator=(&LStack_c0,(L2CValue *)&LStack_f0);
  }
  lib::L2CValue::~L2CValue(&LStack_f0);
  LAB_710015d4dc:
  lib::L2CValue::L2CValue(&LStack_100,0x6e5ec7051);
  lib::L2CValue::L2CValue(&LStack_120,0x18ab103862);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  iVar4 = app::lua_bind::WorkModule__get_param_int_impl
          (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_60,iVar4);
  lib::L2CValue::operator/(&LStack_80,(L2CValue *)&LStack_60);
  lib::L2CValue::L2CValue(&LStack_130,0.0);
  lib::L2CValue::L2CValue(&LStack_140,1.0);
  uVar10 = lib::L2CValue::operator<(&LStack_110,(L2CValue *)&LStack_130);
  if ((uVar10 & 1) == 0) {
  uVar10 = lib::L2CValue::operator<(&LStack_140,(L2CValue *)&LStack_110);
  if ((uVar10 & 1) == 0) {
  lib::L2CValue::L2CValue(&LStack_f0,(L2CValue *)&LStack_110);
  }
  else {
  lib::L2CValue::L2CValue(&LStack_f0,(L2CValue *)&LStack_140);
  }
  }
  else {
  lib::L2CValue::L2CValue(&LStack_f0,(L2CValue *)&LStack_130);
  }
  lib::L2CValue::~L2CValue(&LStack_140);
  lib::L2CValue::~L2CValue(&LStack_130);
  lib::L2CValue::~L2CValue(&LStack_110);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_120);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::operator*(&LStack_a0,(L2CValue *)&LStack_80);
  lib::L2CValue::L2CValue(&LStack_60,1.0);
  lib::L2CValue::operator+(&LStack_60,(L2CValue *)&LStack_150);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::L2CValue(&LStack_60,1.0);
  lib::L2CValue::operator/(&LStack_60,(L2CValue *)&LStack_120);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::operator=(&LStack_90,(L2CValue *)&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_120);
  lib::L2CValue::~L2CValue(&LStack_150);
  lib::L2CValue::L2CValue(&LStack_60,0.0);
  lib::L2CValue::operator+(&LStack_90,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_STATUS_ESCAPE_WORK_FLOAT_MOTION_RATE_PENALTY);
  fVar15 = (float)lib::L2CValue::as_number();
  iVar4 = lib::L2CValue::as_integer();
  app::lua_bind::WorkModule__set_float_impl
  (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),fVar15,iVar4);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::L2CValue(&LStack_160,(L2CValue *)&LStack_b0);
  lib::L2CValue::L2CValue(&LStack_170,(L2CValue *)&LStack_c0);
  LVar5 = lib::L2CValue::L2CValue(&LStack_180,(L2CValue *)&LStack_f0);
  L2CFighterBase::lerp(LVar5,(L2CValue)&LStack_160,(L2CValue)&LStack_170);
  lib::L2CValue::operator=(&LStack_b0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_180);
  lib::L2CValue::~L2CValue(&LStack_170);
  lib::L2CValue::~L2CValue(&LStack_160);
  lib::L2CValue::L2CValue(&LStack_190,(L2CValue *)&LStack_d0);
  lib::L2CValue::L2CValue(&LStack_1a0,(L2CValue *)&LStack_e0);
  LVar5 = lib::L2CValue::L2CValue(&LStack_1b0,(L2CValue *)&LStack_f0);
  L2CFighterBase::lerp(LVar5,(L2CValue)&LStack_190,(L2CValue)&LStack_1a0);
  lib::L2CValue::L2CValue(&LStack_150,0x17ce86529b);
  lib::L2CValue::L2CValue(&LStack_1c0,0);
  uVar10 = lib::L2CValue::as_integer();
  uVar12 = lib::L2CValue::as_integer();
  fVar15 = (float)app::lua_bind::WorkModule__get_param_float_impl
                  (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),uVar10,uVar12);
  lib::L2CValue::L2CValue(&LStack_120,fVar15);
  lib::L2CValue::operator*(&LStack_100,(L2CValue *)&LStack_120);
  lib::L2CValue::operator=(&LStack_d0,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_120);
  lib::L2CValue::~L2CValue(&LStack_1c0);
  lib::L2CValue::~L2CValue(&LStack_150);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_1b0);
  lib::L2CValue::~L2CValue(&LStack_1a0);
  lib::L2CValue::~L2CValue(&LStack_190);
  lib::L2CValue::L2CValue(&LStack_100,FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
  iVar4 = lib::L2CValue::as_integer();
  bVar1 = app::lua_bind::WorkModule__is_flag_impl
          (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),iVar4);
  lib::L2CValue::L2CValue(&LStack_60,(bool)(bVar1 & 1));
  bVar2 = lib::L2CValue::operator.cast.to.bool(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_100);
  if (bVar2) {
  lib::L2CValue::L2CValue(&LStack_60,HIT_STATUS_XLU);
  HVar6 = lib::L2CValue::as_integer();
  app::lua_bind::HitModule__set_whole_impl
    (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),HVar6,0);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
  iVar4 = lib::L2CValue::as_integer();
  app::lua_bind::WorkModule__off_flag_impl(*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),iVar4);
  lib::L2CValue::~L2CValue(&LStack_60);
  }
  lib::L2CAgent::math_toint((L2CValue *)&LStack_b0);
  lib::L2CValue::L2CValue(&LStack_100,_FIGHTER_STATUS_ESCAPE_WORK_INT_HIT_XLU_FRAME);
  iVar4 = lib::L2CValue::as_integer();
  iVar7 = lib::L2CValue::as_integer();
  app::lua_bind::WorkModule__set_int_impl
  (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),iVar4,iVar7);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CAgent::math_toint((L2CValue *)&LStack_d0);
  lib::L2CValue::L2CValue(&LStack_100,_FIGHTER_STATUS_ESCAPE_WORK_INT_HIT_NORMAL_FRAME);
  iVar4 = lib::L2CValue::as_integer();
  iVar7 = lib::L2CValue::as_integer();
  app::lua_bind::WorkModule__set_int_impl
  (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),iVar4,iVar7);
  lib::L2CValue::~L2CValue(&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_60);
  pLVar9 = (L2CValue *)lib::L2CValue::operator[](iVar3);
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_STATUS_KIND_ESCAPE_AIR);
  uVar10 = lib::L2CValue::operator==(pLVar9,(L2CValue *)&LStack_60);
  lib::L2CValue::~L2CValue(&LStack_60);
  if ((uVar10 & 1) != 0) {
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_INSTANCE_WORK_ID_INT_AIR_ESCAPE_ADD_XLU_START_FRAME);
  iVar3 = lib::L2CValue::as_integer();
  iVar3 = app::lua_bind::WorkModule__get_int_impl
            (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),iVar3);
  lib::L2CValue::L2CValue(&LStack_100,iVar3);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::L2CValue(&LStack_60,0);
  uVar10 = lib::L2CValue::operator<(&LStack_60,(L2CValue *)&LStack_100);
  lib::L2CValue::~L2CValue(&LStack_60);
  if ((uVar10 & 1) != 0) {
  lib::L2CValue::L2CValue(&LStack_60,_FIGHTER_STATUS_ESCAPE_WORK_INT_HIT_XLU_FRAME);
  iVar3 = lib::L2CValue::as_integer();
  iVar4 = lib::L2CValue::as_integer();
  app::lua_bind::WorkModule__add_int_impl
      (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),iVar3,iVar4);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::L2CValue(&LStack_60,FIGHTER_STATUS_ESCAPE_AIR_ADD_XLU_START_FRAME);
  iVar3 = lib::L2CValue::as_integer();
  iVar4 = lib::L2CValue::as_integer();
  app::lua_bind::WorkModule__set_int_impl
      (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),iVar3,iVar4);
  lib::L2CValue::~L2CValue(&LStack_60);
  lib::L2CValue::L2CValue(&LStack_60,0);
  lib::L2CValue::L2CValue
      (&LStack_120,FIGHTER_INSTANCE_WORK_ID_INT_AIR_ESCAPE_ADD_XLU_START_FRAME);
  iVar3 = lib::L2CValue::as_integer();
  iVar4 = lib::L2CValue::as_integer();
  app::lua_bind::WorkModule__set_int_impl
      (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),iVar3,iVar4);
  lib::L2CValue::~L2CValue(&LStack_120);
  lib::L2CValue::~L2CValue(&LStack_60);
  }
  lib::L2CValue::~L2CValue(&LStack_100);
  }
  fVar15 = (float)lib::L2CValue::as_number();
  app::lua_bind::FighterWorkModuleImpl__calc_escape_air_slide_param_impl
  (*(BattleObjectModuleAccessor **)(pLVar8 + 0x10),fVar15);
  lib::L2CValue::~L2CValue(&LStack_f0);
  lib::L2CValue::~L2CValue(&LStack_e0);
  lib::L2CValue::~L2CValue(&LStack_d0);
  lib::L2CValue::~L2CValue(&LStack_c0);
  lib::L2CValue::~L2CValue(&LStack_b0);
  lib::L2CValue::~L2CValue(&LStack_a0);
  lib::L2CValue::~L2CValue(&LStack_90);
  lib::L2CValue::~L2CValue(&LStack_80);
  lib::L2CValue::~L2CValue(&LStack_70);
  return;
}

//


/* app::lua_bind::FighterWorkModuleImpl__calc_escape_air_slide_param_impl(app::BattleObjectModuleAcc essor*,
   float) */

   void app::lua_bind::FighterWorkModuleImpl__calc_escape_air_slide_param_impl
   (BattleObjectModuleAccessor *param_1,float param_2){
  uint uVar1;
  float *pfVar2;
  ulong uVar3;
  long lVar4;
  ulong uVar5;
  long *plVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;

  plVar6 = *(long **)(param_1 + 0x50);
  uVar1 = 0x4b;
  if (*(uint *)(plVar6 + 8) != 0x4c) {
  uVar1 = *(uint *)(plVar6 + 8);
  }
  if (uVar1 < 0x5e) {
  uVar3 = (ulong)*(int *)(lib::Singleton<app::FighterParamAccessor2>::instance_ +
                  (long)(int)uVar1 * 0xc + 0x14f4);
  lVar4 = *(long *)(*(long *)(lib::Singleton<app::FighterParamAccessor2>::instance_ + 0x10) + 8);
  uVar5 = (*(long *)(*(long *)(lib::Singleton<app::FighterParamAccessor2>::instance_ + 0x10) +
            0x10) - lVar4 >> 3) * -0xf4898d5f85bb395;
  if (uVar3 <= uVar5 && uVar5 - uVar3 != 0) {
  lVar4 = lVar4 + uVar3 * 0x218;
  fVar8 = *(float *)(lVar4 + 0x88);
  fVar9 = *(float *)(lVar4 + 0x80);
  fVar10 = *(float *)(lVar4 + 0x8c);
  fVar11 = (1.0 - param_2) * *(float *)(lVar4 + 0x7c) + *(float *)(lVar4 + 0x84) * param_2;
  fVar7 = (float)(**(code **)(*plVar6 + 0x270))(plVar6,0x1dfb01ad43,0);
  fVar7 = (fVar10 - fVar11) /
    ((fVar10 * fVar10 - fVar11 * fVar11) *
    (1.0 / (((1.0 - param_2) * fVar9 + fVar8 * param_2) * fVar7)) * 0.5);
  pfVar2 = (float *)FUN_71003ca970(&DAT_00001b18 + *(long *)(plVar6[7] + 8),0x166ed053cc);
  *pfVar2 = fVar11;
  pfVar2 = (float *)FUN_71003ca970(&DAT_00001b18 + *(long *)(plVar6[7] + 8),0x163c0a9bdf);
  *pfVar2 = (fVar10 - fVar11) / (float)(int)fVar7;
  pfVar2 = (float *)FUN_71003ca970(&DAT_00001b18 + *(long *)(plVar6[7] + 8),0x15f2c6719b);
  *pfVar2 = (float)(int)fVar7;
  return;
  }
          /* WARNING: Subroutine does not return */
  std::__1::__vector_base_common<true>::__throw_out_of_range();
  }
          /* WARNING: Subroutine does not return */
  abort();
}

//

