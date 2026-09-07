
use std::env::consts::OS;
use std::env::consts::ARCH;


use std::{env};

 #[derive(Debug)]
pub struct System {
    os: String,
    arch: String,

}

struct etc_password {

}

use std::fs::{File};

pub fn collect_system() -> System{
    // getting to know the target
    let _os = OS.to_lowercase();
    let _arch = ARCH.to_lowercase();

    System {
        os : _os,
        arch : _arch,
    }

}

pub fn collect_passwd() -> Result<(),std::io::Error> {

    let etc_passwd = File::open("/etc/passwd")?;
    let etc_passwd_metadata = etc_passwd.metadata()?;
    println!("file scanned : /etc/passwd ");
    println!("Metadata : {:#?}", etc_passwd_metadata);

    Ok(())



}

pub fn collect_shadow() -> Result<(),std::io::Error> {
    let etc_shadow = File::open("/etc/shadow")?;
    let etc_shadow_metadata = etc_shadow.metadata()?;
    println!("file scanned : /etc/shadow ");
    println!(" Metadata : {:#?}", etc_shadow_metadata);
    Ok(())

}

pub fn collect_sshd_config()-> Result<(),std::io::Error> {

    let etc_sshd_config = File::open("/etc/ssh/sshd_config")?;
    let etc_sshd_config_metadata = etc_sshd_config.metadata()?;
    println!("file scanned :  /etc/ssh/sshd_config");
    println!("Metadata :  {:#?}", etc_sshd_config_metadata);
    Ok(())
}