use std::path::Path;
use std::process::Command;
use crate::sys::distros::Distro;

pub fn execute_grub_repair(chroot_path: &Path, distro: &dyn Distro) -> std::io::Result<()>{
    // Rebuild the system initramfs images first inside the chroot environment
    let img_args = distro.initramfs_cmd();
    if !img_args.is_empty() {
        Command::new("chroot")
            .arg(chroot_path)
            .args(&img_args)
            .status()?;
    }

    // Resolve the targeted GRUB config path property dynamically from the trait configuration
    let relative_cfg = distro.grub_config_path();

    // Strip leading slash if present to safely append path structures relative to the host mount directory location
    let relative_cfg_cleaned = relative_cfg.strip_prefix("/").unwrap_or(relative_cfg);
    let absolute_target_cfg = chroot_path.join(relative_cfg_cleaned);

    //  Re-generate the GRUB primary settings mapping file
    let status = Command::new("chroot")
        .arg(chroot_path)
        .args([
            "grub-mkconfig",
            "-o",
            relative_cfg.to_str().unwrap()
        ])
        .status()?;

    if !status.success() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "grub-mkconfig failed"));
    }

    // Run post-processing macros if demanded by the distribution platform architecture rules
    distro.post_grub_hook(chroot_path)?;

    Ok(())
}