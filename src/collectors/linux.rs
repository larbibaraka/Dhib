
use std::env::consts::OS;
use std::env::consts::ARCH;

use std::time::SystemTime;
use std::fs::{File};
use std::io::{BufRead, BufReader, Read};
use std::os::unix::fs::PermissionsExt;

use std::os::linux::fs::MetadataExt;
use std::path::Path;

#[derive(Debug)]
pub struct System {
    os: String,
    arch: String,

}
#[derive(Debug)]
pub struct Permissions {
    mode : u32,
    user_owner : u32,
    group_owner : u32,
}
#[derive(Debug)]
pub struct Timestamps {
    modified : SystemTime,
    accessed : SystemTime

}

#[derive(Debug)]
pub struct MetadataStruct {
    file_permissions: Permissions,
    is_symlink : bool,
    length : u64,
    timestamps :Timestamps
}

#[derive(Debug)]
pub struct PasswdEntry {
    username : String,
    password : String,
    comment : String,
    user_id : String,
    group_id : String,
    home_dir : String,
    login_shell : String,
}

#[derive(Debug)]
pub struct ShadowEntry {
    username: String,
    encrypted_password: String,
    last_password_changed: String,
    minimum_password_changed: String,
    maximum_password_changed: String,
    password_warning_period: String,
    password_inactivity_period: String,
    account_expiration_date: String,
    reserved: String
}


#[derive(Debug)]
pub struct SshConfigEntry  {
    include : String,
    kbd_interactive_authentication : String,
    use_pam : String,
    x11_forwarding : String,
    print_motd : String,
    accept_env : Vec<String>,
    sub_system : Vec<String>,
}



pub fn collect_system() -> System{
    // getting to know the target
    let _os = OS.to_lowercase();
    let _arch = ARCH.to_lowercase();
    System {
        os : _os,
        arch : _arch,
    }
}

fn collect_file_metadata(path : &str) -> Result<MetadataStruct, std::io::Error> {
    let file = File::open(path)?;
    let metadata = file.metadata()?;

    let evidence = MetadataStruct {
        file_permissions: Permissions {
            // 0o100644 (-rw-r--r--)
            mode : metadata.permissions().mode() ,
            user_owner : metadata.st_uid(),
            group_owner : metadata.st_gid()
        },
        is_symlink : metadata.file_type().is_symlink(),
        length : metadata.len(),
        timestamps : Timestamps {
            accessed : metadata.accessed()?,
            modified : metadata.modified()?
        }
    };
    Ok(evidence)
}


fn file_reader(path : &str) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn collect_passwd_file_contents(path : &str) -> Result<Vec<PasswdEntry>, std::io::Error> {
     let file = file_reader(path)?;
     let mut file_struct_vec : Vec<PasswdEntry> = Vec::new();
     for line in file.lines() {
         let fields = line.split(':').collect::<Vec<&str>>();
         if(fields.len() != 7) {
             return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("/etc/passwd contains malformed entry at line {}", line)));
         }
         let username =  fields.get(0).map(|value| value.to_string()).unwrap_or_default();
         let password =  fields.get(1).map(|value| value.to_string()).unwrap_or_default();
         let user_id  =   fields.get(2).map(|value| value.to_string()).unwrap_or_default();
         let group_id =   fields.get(3).map(|value| value.to_string()).unwrap_or_default();
         let comment =  fields.get(4).map(|value| value.to_string()).unwrap_or_default();
         let home_dir =  fields.get(5).map(|value| value.to_string()).unwrap_or_default();
         let login_shell =  fields.get(6).map(|value| value.to_string()).unwrap_or_default();

         let file_struct = PasswdEntry {
             username ,
             password ,
             user_id ,
             group_id ,
             comment ,
             home_dir ,
             login_shell
         };
         file_struct_vec.push(file_struct);
     }
     Ok(file_struct_vec)
}


fn collect_shadow_file_contents(path : &str) -> Result<Vec<ShadowEntry>, std::io::Error> {
    let file = file_reader(path)?;
    let mut file_struct_vec : Vec<ShadowEntry> = Vec::new();

    for line in file.lines() {

        let fields = line.split(':').collect::<Vec<&str>>();
        if(fields.len() != 9) {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("/etc/shadow contains malformed entry at line {}", line)));
        }
        let username =  fields.get(0).map(|value| value.to_string()).unwrap_or_default();
        let encrypted_password =  fields.get(1).map(|value| value.to_string()).unwrap_or_default();
        let last_password_changed  =  fields.get(2).map(|value| value.to_string()).unwrap_or_default();
        let minimum_password_changed =   fields.get(3).map(|value| value.to_string()).unwrap_or_default();
        let maximum_password_changed =  fields.get(4).map(|value| value.to_string()).unwrap_or_default();
        let password_warning_period =  fields.get(5).map(|value| value.to_string()).unwrap_or_default();
        let password_inactivity_period =  fields.get(6).map(|value| value.to_string()).unwrap_or_default();
        let account_expiration_date = fields.get(7).map(|value| value.to_string()).unwrap_or_default();
        let reserved =fields.get(8).map(|value| value.to_string()).unwrap_or_default();

        let file_struct = ShadowEntry {
            username ,
            encrypted_password ,
            last_password_changed ,
            minimum_password_changed ,
            maximum_password_changed ,
            password_warning_period ,
            password_inactivity_period,
            account_expiration_date,
            reserved
        };
        file_struct_vec.push(file_struct);
    }
    Ok(file_struct_vec)
}

fn collect_sshd_config_file_contents(path : &str) -> Result<SshConfigEntry, std::io::Error> {
    let file = file_reader(path)?;

    let mut include = String::new();
    let mut kbd_interactive_authentication = String::new();
    let mut use_pam = String::new();
    let mut x11_forwarding = String::new();
    let mut print_motd = String::new();
    let mut accept_env: Vec<String> = Vec::new();
    let mut sub_system: Vec<String> = Vec::new();
    for line in file.lines() {
        if(!line.is_empty() && !line.starts_with('#')) {
            let mut parts = line.split_whitespace();
            let directive = parts.next();
            let arguments: Vec<&str> = parts.collect();


            match directive {
                Some("Include") => {
                    include = arguments[0].to_string();
                }
                Some("KbdInteractiveAuthentication") => {
                    kbd_interactive_authentication = arguments[0].to_string();
                }
                Some("UsePam") => {
                    use_pam = arguments[0].to_string();
                }
                Some("X11Forwarding") => {
                    x11_forwarding = arguments[0].to_string();
                }
                Some("PrintMotd") => {
                    print_motd = arguments[0].to_string();
                }
                Some("AcceptEnv") => {
                    accept_env.extend(arguments.into_iter().map(String::from));
                }
                Some("SubSystem") => {
                    sub_system.extend(arguments.into_iter().map(String::from));
                }

                _ => {}
            }




        }


    }

    let my_struct = SshConfigEntry {
        include,
        kbd_interactive_authentication,
        use_pam,
        x11_forwarding,
        print_motd,
        accept_env,
        sub_system,
    };

    Ok(my_struct)

}


pub fn collect_passwd_contents() -> Result<Vec<PasswdEntry>, std::io::Error> {
    let pass = collect_passwd_file_contents("/etc/passwd")?;
    Ok(pass)
}

pub fn collect_shadow_contents() -> Result<Vec<ShadowEntry>, std::io::Error> {
    let shadow = collect_shadow_file_contents("/etc/shadow")?;
    Ok(shadow)
}

pub fn collect_sshd_config_contents() -> Result <(), std::io::Error> {
    let ssh_config = collect_sshd_config_file_contents("/etc/ssh/sshd_config")?;
    println!("{:#?}", ssh_config);



    Ok(())
}



pub fn collect_passwd() -> Result<MetadataStruct,std::io::Error> {
    let file = collect_file_metadata("/etc/passwd")?;
    println!("file scanned : /etc/passwd ");
    Ok(file)
}

pub fn collect_shadow() -> Result<MetadataStruct,std::io::Error> {
    let file = collect_file_metadata("/etc/shadow")?;
    println!("file scanned : /etc/shadow ");
    Ok(file)
}

pub fn collect_sshd_config()-> Result<MetadataStruct,std::io::Error> {
    let file = collect_file_metadata("/etc/ssh/sshd_config")?;
    println!("file scanned :  /etc/ssh/sshd_config");
    Ok(file)
}





