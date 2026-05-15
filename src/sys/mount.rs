use std::fs;
use std::path::Path;
use std::process::Command;

const MOUNT_PATH: &str = "/mnt";
const MOUNT_EFI_PATH: &str = "/mnt/boot/efi";

/// Mount `device` to `/mnt`.
pub fn mount(device: &str) {
    let status = Command::new("mount")
        .args(&[device, MOUNT_PATH])
        .status()
        .expect("failed to execute mount");
    if !status.success() {
        panic!("failed to mount {} to {}", device, MOUNT_PATH);
    }
}

/// Mount `efi_device` to `/mnt/boot/efi`, creating the directory if needed.
pub fn mount_efi(efi_device: &str) {
    let efi_path = Path::new(MOUNT_EFI_PATH);
    if !efi_path.exists() {
        fs::create_dir_all(efi_path).expect("failed to create efi directory");
    }
    let status = Command::new("mount")
        .args(&[efi_device, MOUNT_EFI_PATH])
        .status()
        .expect("failed to execute mount for EFI");
    if !status.success() {
        panic!("failed to mount EFI {} to {}", efi_device, MOUNT_EFI_PATH);
    }
}

/// Bind-mount /dev, /proc, /sys, /run into the chroot.
pub fn mount_bind() {
    let binds = ["/dev", "/proc", "/run", "/sys"];
    for bind in binds {
        let target = format!("{}{}", MOUNT_PATH, bind);
        let status = Command::new("mount")
            .args(&["--bind", bind, &target])
            .status()
            .expect("failed to execute mount --bind");
        if !status.success() {
            panic!("failed to bind mount {} to {}", bind, target);
        }
    }
}

/// Recursively unmount everything under `mount_dir`.
pub fn umount(mount_dir: &str) {
    let status = Command::new("umount")
        .args(&["-R", mount_dir])
        .status()
        .expect("failed to execute umount");
    if !status.success() {
        panic!("failed to umount {}", mount_dir);
    }
}