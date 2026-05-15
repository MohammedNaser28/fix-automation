// src/screens/mod.rs

use ratatui::Frame;
use crate::app::{App, CurrentScreen};

// 1. You must declare the modules so Rust finds the files
pub mod welcome;
pub mod select_root;
pub mod select_efi;
pub mod confirm;
// pub mod fstab_editor; // Keep commented until you create the file

pub fn render(f: &mut Frame, app: &mut App) {
    // 2. This match statement decides which file's render function to call
    match app.current_screen {
        CurrentScreen::Welcome => welcome::render(f, app),

        CurrentScreen::SelectRoot => select_root::render(f, app),

        CurrentScreen::SelectEfi => select_efi::render(f, app),

        CurrentScreen::Confirm => confirm::render(f, app),

        // 3. Any screen not listed here will show the "Work in Progress" block
        _ => {
            let block = ratatui::widgets::Block::default()
                .title(" Work in Progress ")
                .borders(ratatui::widgets::Borders::ALL)
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));
            f.render_widget(block, f.size());
        }
    }
}