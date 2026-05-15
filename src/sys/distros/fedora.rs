use std::path::{Path, PathBuf};
use crate::sys::distros::Distro;

pub struct Fedora;

impl Distro for Fedora {
    fn name(&self) -> &'static str { "Fedora Linux" }

    fn grub_config_path(&self) -> &Path {
        // Fedora and RHEL variants use the grub2 config layout paradigm
        Path::new("/boot/grub2/grub.cfg")
    }

    fn initramfs_cmd(&self) -> Vec<&'static str> {
        vec!["dracut", "--regenerate-all", "--force"]
    }

    fn post_grub_hook(&self, _chroot_path: &Path) -> std::io::Result<()> {
        Ok(())
    }
}