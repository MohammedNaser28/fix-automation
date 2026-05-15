// src/screens/mod.rs

use ratatui::Frame;
use crate::app::{App, CurrentScreen};

pub mod welcome;
pub mod select_root;
pub mod select_efi;
pub mod confirm;
pub mod action_menu;
pub mod exec_log;

pub fn render(f: &mut Frame, app: &mut App) {
    match app.current_screen {
        CurrentScreen::Welcome => welcome::render(f, app),
        CurrentScreen::SelectRoot => select_root::render(f, app),
        CurrentScreen::SelectEfi => select_efi::render(f, app),
        CurrentScreen::Confirm => confirm::render(f, app),
        CurrentScreen::ActionMenu => action_menu::render(f, app),
        CurrentScreen::ExecLog => exec_log::render(f, app),
    }
}