
use std::env::consts::OS;
use std::env::consts::ARCH;

use std::time::SystemTime;
use std::fs::{File};

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
pub struct FileStruct {
    file_permissions: Permissions,
    is_symlink : bool,
    length : u64,
    timestamps :Timestamps
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

fn collect_file_metadata(path : &str) -> Result<FileStruct, std::io::Error> {
    let file = File::open(path)?;
    let metadata = file.metadata()?;

    let evidence = FileStruct {
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

pub fn collect_passwd() -> Result<FileStruct,std::io::Error> {
    let file = collect_file_metadata("/etc/passwd")?;
    println!("file scanned : /etc/passwd ");
    Ok(file)
}

pub fn collect_shadow() -> Result<FileStruct,std::io::Error> {
    let file = collect_file_metadata("/etc/passwd")?;
    println!("file scanned : /etc/shadow ");
    Ok(file)
}

pub fn collect_sshd_config()-> Result<FileStruct,std::io::Error> {
    let file = collect_file_metadata("/etc/ssh/sshd_config")?;
    println!("file scanned :  /etc/ssh/sshd_config");
    Ok(file)
}



