
use smash::app::{sv_system, sv_animcmd::{frame, wait}, self, lua_bind::*};
use smash::phx::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::utility::*;
use smash_script::*;
use smashline::*;
use smashline::*;
use smash_script::macros::*;
use crate::utils::hdr;
use crate::vars::*;


#[acmd_script( agent = "falco", script = "game_specialairsend" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_air_s_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {  }
}

#[acmd_script( agent = "falco", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 3.0, 367, 60, 0, 60, 5.5, 3.2, -3.1, -1.5, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("hip"), 3.0, 80, 100, 80, 0, 5.5, 3.2, -3.1, -1.5, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    for _ in 0..7 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("hip"), 2.0, 367, 60, 0, 60, 4.0, 3.2, -3.1, -1.5, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            ATTACK(fighter, 1, 0, Hash40::new("hip"), 2.0, 65, 100, 80, 0, 4.0, 3.2, -3.1, -1.5, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 2.0, 75, 220, 0, 70, 6.0, 4.2, -3.1, -1.5, None, None, None, 1.7, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "falco", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 84, 48, 0, 103, 7.5, 0.0, 6.5, 2.0, None, None, None, 0.6, 1.15, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {  }frame(lua_state, 51.0);
    if is_excute(fighter) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
    }
    
}

#[acmd_script( agent = "falco", script = "game_specialairlw" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let shine_vec = Vector3f { x: 0.25, y: 0.0, z: 1.0 };
        KineticModule::mul_speed(boma, &shine_vec, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let shineGravity = Vector3f { x: 1.0, y: 0.1588, z: 1.0 };
        KineticModule::mul_accel(boma, &shineGravity, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 84, 48, 0, 103, 7.5, 0.0, 6.5, 2.0, None, None, None, 0.6, 1.15, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {  }frame(lua_state, 51.0);
    if is_excute(fighter) {
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
    }
    
}

#[acmd_script( agent = "falco", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_lw_r_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 84, 48, 0, 103, 7.5, 0.0, 6.5, 2.0, None, None, None, 0.6, 1.15, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {  }frame(lua_state, 51.0);
    if is_excute(fighter) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
    }
    
}

#[acmd_script( agent = "falco", script = "game_specialairlw" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_air_lw_r_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let shine_vec = Vector3f { x: 0.25, y: 0.0, z: 1.0 };
        KineticModule::mul_speed(boma, &shine_vec, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let shineGravity = Vector3f { x: 1.0, y: 0.1588, z: 1.0 };
        KineticModule::mul_accel(boma, &shineGravity, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 84, 48, 0, 103, 7.5, 0.0, 6.5, 2.0, None, None, None, 0.6, 1.15, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {  }frame(lua_state, 51.0);
    if is_excute(fighter) {
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        falco_special_air_s_end_game,
        falco_special_hi_game,
        falco_special_lw_game,
        falco_special_air_lw_game,
        falco_special_lw_r_game,
        falco_special_air_lw_r_game,
    );
}
