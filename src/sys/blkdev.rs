use serde::Deserialize;
use std::process::Command;

// 1. The struct used by your UI (no Serde tags needed here)
#[derive(Clone, Debug)]
pub struct DiskInfo {
    pub name: String,
    pub size: String,
    pub fstype: Option<String>,
    pub label: Option<String>,
    pub uuid: Option<String>,
    pub mountpoint: Option<String>,
    pub is_efi: bool,
    pub contents: Option<String>,
}

// 2. Structs used ONLY for parsing lsblk JSON
#[derive(Deserialize)]
struct LsblkOutput {
    blockdevices: Vec<BlockDevice>,
}

#[derive(Deserialize)]
struct BlockDevice {
    name: String,
    size: String,
    fstype: Option<String>,
    label: Option<String>,
    uuid: Option<String>,
    mountpoint: Option<String>,
    children: Option<Vec<BlockDevice>>,
}

pub fn get_disks() -> Vec<DiskInfo> {
    let output = Command::new("lsblk")
        .args(["--json", "-o", "NAME,SIZE,FSTYPE,LABEL,UUID,MOUNTPOINT"])
        .output()
        .expect("failed to execute lsblk");

    let decoded: LsblkOutput = serde_json::from_slice(&output.stdout).unwrap_or(LsblkOutput {
        blockdevices: vec![],
    });

    let mut disks = Vec::new();

    for dev in decoded.blockdevices {
        // We look at children because 'nvme0n1' is the disk,
        // but 'nvme0n1p1' (the child) is the partition we care about.
        if let Some(partitions) = dev.children {
            for part in partitions {
                let is_efi = part.fstype.as_deref() == Some("vfat");

                disks.push(DiskInfo {
                    name: part.name,
                    size: part.size,
                    fstype: part.fstype,
                    label: part.label,
                    uuid: part.uuid,
                    mountpoint: part.mountpoint,
                    is_efi,
                    // We initialize this as None.
                    // You'll fill this in later when you scan the EFI partition.
                    contents: if is_efi {
                        Some("Scanning...".into())
                    } else {
                        None
                    },
                });
            }
        }
    }
    disks
}
