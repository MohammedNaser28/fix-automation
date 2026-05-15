use std::fs;
use std::path::Path;
use std::process::Command;
use serde::Deserialize;

const MOUNT_PATH: &str = "/mnt/";
const MOUNT_EFI_PATH: &str = "/mnt/boot/efi/";
pub fn mount(mount_dir: &str) {

    let status = Command::new("mount").args(&["-t", mount_dir , MOUNT_PATH]).status().expect("failed to execute process");
    if !status.success() {
        panic!("failed to mount device");
    }
}
pub fn mount_efi(efi_dir: &str)  {

    let efi_path = Path::new(MOUNT_EFI_PATH);
    if !efi_path.exists() {
        fs::create_dir_all(efi_path).expect("failed to create efi directory");
    }
    let status = Command::new("mount").args(&[efi_dir,MOUNT_EFI_PATH]).status().expect("failed to execute process");
    if  !status.success()  {
            panic!("failed to create efi directory");

    }
}

pub fn mount_bind() {
    let binds  = ["/dev", "/proc","/run","/sys"];
    for bind in binds {
        let target = format!("{}{}", MOUNT_PATH, bind);
        let status = Command::new("mount")
            .args(&["--bind", bind, &target])
            .status()
            .expect("failed to execute mount bind process");

        if !status.success() {
            panic!("failed to bind mount {} to {}", bind, target);
        }
    }
}

pub fn umount(mount_dir: &str) {
    let status = Command::new("umount").args(&["-R",  mount_dir]).status().expect("failed to execute process");
if !status.success(){
    panic!("failed to umount {}", mount_dir);
}
}