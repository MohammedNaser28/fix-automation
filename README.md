# Fix-Automation

This project aim to solve spcific problem
- grub failuer issue
    - it remake grub config after resize or install fresh windows
- fix `fstab` issues
    - it comapre the UUID if there's something wrong after fresh install or re partioning 

### I wish to can add those
- [ ] add methods to send logs of problems to AI or specific links to solve it 
- [ ] check errors and detect solutions 
    - [ ] gnome failuer or hyprland 
    - [ ] reset passwords


## Structure
```tree
grub-rescue/
├── Cargo.toml
├── src/
│   ├── main.rs                  ← terminal init, event loop, render dispatch
│   ├── app.rs                   ← App struct, all state, screen enum
│   │
│   ├── screens/
│   │   ├── mod.rs
│   │   ├── welcome.rs           ← screen 1
│   │   ├── select_root.rs       ← screen 2
│   │   ├── select_efi.rs        ← screen 3
│   │   ├── confirm.rs           ← screen 4
│   │   ├── action_menu.rs       ← screen 5
│   │   ├── exec_log.rs          ← screen 6
│   │   ├── fstab_editor.rs      ← screen 7
│   │   ├── partition_mgr.rs     ← screen 8
│   │   ├── log_export.rs        ← screen 9
│   │   └── result.rs            ← screen 10
│   │
│   ├── sys/                     ← all system calls, no UI here
│   │   ├── mod.rs
│   │   ├── blkdev.rs            ← lsblk parsing → DiskInfo structs
│   │   ├── mount.rs             ← mount / umount / bind mounts
│   │   ├── chroot.rs            ← chroot + env setup
│   │   ├── firmware.rs          ← UEFI vs BIOS detection
│   │   ├── network.rs           ← check if network is up, get IP
│   │   ├── grub.rs              ← grub-install + grub-mkconfig
│   │   ├── fstab.rs             ← UUID detection + fstab generation
│   │   ├── windows.rs           ← NTFS mount + EFI backup detection
│   │   ├── logs.rs              ← session log collector + paste upload
│   │   └── distro/
│   │       ├── mod.rs           ← Distro trait + detect()
│   │       ├── arch.rs
│   │       ├── debian.rs
│   │       ├── fedora.rs
│   │       └── unknown.rs
│   │
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── theme.rs             ← color constants (THEME struct)
│   │   ├── widgets/
│   │   │   ├── mod.rs
│   │   │   ├── statusbar.rs     ← top status bar (shared across screens)
│   │   │   ├── bottombar.rs     ← keybind hints (shared across screens)
│   │   │   ├── sidebar.rs       ← left sidebar (shared across screens)
│   │   │   ├── partition_table.rs ← reusable disk/partition table widget
│   │   │   └── disk_bar.rs      ← visual partition bar widget
│   │   └── layout.rs            ← common layout builders
│   │
│   └── events/
│       ├── mod.rs
│       └── handler.rs           ← all key handling per screen
```


## Contrbuting
 feel free to add or suggest something
