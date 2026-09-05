
use std::env::consts::OS;
use std::env::consts::ARCH;


use std::{env};

struct System {
    os: String,
    arch: String,

}

use std::fs::{File};
pub fn collect_system(){
    // getting to know the target
    let _os = OS.to_lowercase();
    let _arch = ARCH.to_lowercase();

    System {
        os : _os,
        arch : _arch,
    };

}

pub fn collect_passwd() {



}

pub fn collect_shadow(){

}

pub fn collect_sshd_config(){

}