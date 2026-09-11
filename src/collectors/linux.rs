
use std::env::consts::OS;
use std::env::consts::ARCH;

use std::time::SystemTime;
use std::fs::{File};

use std::os::unix::fs::PermissionsExt;

use std::os::linux::fs::MetadataExt;
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
pub struct PasswordEvidence {
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

pub fn collect_passwd() -> Result<PasswordEvidence,std::io::Error> {

    let etc_passwd = File::open("/etc/passwd")?;
    let metadata = etc_passwd.metadata()?;
    println!("file scanned : /etc/passwd ");


    let evidence = PasswordEvidence {
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

// pub fn collect_shadow() -> Result<(),std::io::Error> {
//     let etc_shadow = File::open("/etc/shadow")?;
//     let etc_shadow_metadata = etc_shadow.metadata()?;
//     println!("file scanned : /etc/shadow ");
//     println!(" Metadata : {:#?}", etc_shadow_metadata);
//     Ok(())
// 
// }
// 
// pub fn collect_sshd_config()-> Result<(),std::io::Error> {
// 
//     let etc_sshd_config = File::open("/etc/ssh/sshd_config")?;
//     let etc_sshd_config_metadata = etc_sshd_config.metadata()?;
//     println!("file scanned :  /etc/ssh/sshd_config");
//     println!("Metadata :  {:#?}", etc_sshd_config_metadata);
//     Ok(())
// }