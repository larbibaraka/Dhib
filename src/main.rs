
use std::{env};



use std::fs::{File};
use crate::collectors::linux;
// use std::io::prelude::*;


mod collectors;

fn main()  -> std::io::Result<()>{
    // args part of the code
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();


    let system = collectors::linux::collect_system();

    println!("{:?}", system);



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




    println!("===========================");



    let evidance = collectors::linux::collect_passwd()?;
    println!("{:?}", evidance);
    // println!("===========================");
    // collectors::linux::collect_shadow()?;
    // println!("===========================");
    // collectors::linux::collect_sshd_config()?;
    







    Ok(())







}
