use std::path::Path;

/// Returns `true` if the system booted in UEFI mode.
/// Detection is based on the presence of the EFI firmware interface sysfs directory.
pub fn is_uefi() -> bool {
    Path::new("/sys/firmware/efi").exists()
}
