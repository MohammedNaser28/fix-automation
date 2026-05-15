use std::path::Path;
use std::process::Command;

pub trait Distro {
    /// Returns the user-facing name of the distribution (e.g., "Arch Linux").
    fn name(&self) -> &'static str;

    /// Returns the absolute target path where the GRUB configuration file should live.
    fn grub_config_path(&self) -> &Path;

    /// Returns the specific binary and arguments required to rebuild the initramfs/initrd inside a chroot environment.
    /// Example: ["mkinitcpio", "-P"] or ["dracut", "--regenerate-all", "--force"]
    fn initramfs_cmd(&self) -> Vec<&'static str>;

    /// Handles any unique logic required post-installation (e.g., executing update-grub on Debian variants).
    fn post_grub_hook(&self, chroot_path: &Path) -> std::io::Result<()>;
}

pub mod arch;
pub mod debian;
pub mod fedora;
pub mod unknown;

/// Inspects a mounted root partition filesystem to dynamically identify the underlying distribution family.
pub fn detect(target_mount: &Path) -> Box<dyn Distro> {
    let os_release_path = target_mount.join("etc/os-release");

    if let Ok(content) = std::fs::read_to_string(os_release_path) {
        if content.contains("ID=arch") {
            return Box::new(arch::ArchLinux);
        } else if content.contains("ID=debian") || content.contains("ID_LIKE=debian") || content.contains("ID=ubuntu") {
            return Box::new(debian::Debian);
        } else if content.contains("ID=fedora") || content.contains("ID_LIKE=rhel") {
            return Box::new(fedora::Fedora);
        }
    }
    Box::new(unknown::UnknownDistro::new(target_mount))}