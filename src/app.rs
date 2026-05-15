use ratatui::widgets::TableState;
use crate::sys::blkdev::DiskInfo;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CurrentScreen {
    Welcome,
    SelectRoot,
    SelectEfi,
    Confirm,
    ActionMenu,
    ExecLog,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ConfirmFocus {
    Confirm,
    Back,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub should_quit: bool,
    pub is_uefi: bool,
    pub detected_distro: Option<String>,
    pub confirm_focus: ConfirmFocus,

    // System data
    pub disks: Vec<DiskInfo>,
    pub selected_root: Option<DiskInfo>,
    pub selected_efi: Option<DiskInfo>,

    // UI state
    pub table_state: TableState,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Welcome,
            should_quit: false,
            is_uefi: true,
            detected_distro: None,
            confirm_focus: ConfirmFocus::Confirm,

            disks: Vec::new(),
            selected_root: None,
            selected_efi: None,

            table_state: TableState::default(),
        }
    }

    pub fn select_next(&mut self) {
        if self.disks.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => if i >= self.disks.len() - 1 { 0 } else { i + 1 },
            None    => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn select_previous(&mut self) {
        if self.disks.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => if i == 0 { self.disks.len() - 1 } else { i - 1 },
            None    => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn toggle_confirm_buttons(&mut self) {
        self.confirm_focus = match self.confirm_focus {
            ConfirmFocus::Confirm => ConfirmFocus::Back,
            ConfirmFocus::Back    => ConfirmFocus::Confirm,
        };
    }
}