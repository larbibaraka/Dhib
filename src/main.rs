
use std::{env};



use std::fs::{File};

// use std::io::prelude::*;


mod collectors;

fn main()  -> std::io::Result<()>{
    // args part of the code
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();


 


    if args_len  != 3 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "in order this program to work you need three arguments eg usage : dhib scan linux"));

    }
    let command = &args[1];
    let target = &args[2];
    // checking args we need for now only scan and linux target
    if command.is_empty() || !command.eq("scan") {

        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "you need to pass 'scan' command in order this tool to work"));
    }

    if target.is_empty() || !target.eq("linux") {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "target not supported"));
    }



    /*
    we need to inspect this :
        - /etc/passwd
        - /etc/shadow
        - /etc/ssh/sshd_config
     */

    let etc_passwd = File::open("/etc/passwd")?;
    let etc_passwd_metadata = etc_passwd.metadata()?;
    println!("file scanned : /etc/passwd ");
    println!("Metadata : {:#?}", etc_passwd_metadata);


    println!("===========================");

    let etc_shadow = File::open("/etc/shadow")?;
    let etc_shadow_metadata = etc_shadow.metadata()?;
    println!("file scanned : /etc/shadow ");
    println!(" Metadata : {:#?}", etc_shadow_metadata);
    println!("===========================");


    let etc_sshd_config = File::open("/etc/ssh/sshd_config")?;
    let etc_sshd_config_metadata = etc_sshd_config.metadata()?;
    println!("file scanned :  /etc/ssh/sshd_config");
    println!("Metadata :  {:#?}", etc_sshd_config_metadata);


    Ok(())







}
