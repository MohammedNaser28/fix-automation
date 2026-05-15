use ratatui::{
    layout::{Alignment, Constraint, Layout},
    widgets::{Block, Paragraph, Wrap,Borders
    },
    Frame,
};
use crate::app::App;
use crate::ui::theme::DRACULA;
use crate::app::ConfirmFocus;
pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.size());

    // Summary Text
    let root = app.selected_root.as_ref().unwrap();
    let efi = app.selected_efi.as_ref().unwrap();
    let distro = app.detected_distro.as_deref().unwrap_or("unknown");
    let text = format!(
        "root partition  /dev/{} {} {}\n\
         efi partition   /dev/{} {} {}\n\
         distro detected Arch Linux [arch]\n\
         firmware        UEFI + BIOS fallback\n\
         grub target     x86_64-efi + i386-pc",
        root.name, root.fstype.as_deref().unwrap(), root.size,
        efi.name, efi.fstype.as_deref().unwrap(), efi.size
    );

    let summary = Paragraph::new(text)
        .block(Block::default().title("confirm — review before repair"))
        .wrap(Wrap { trim: false });

    f.render_widget(summary, chunks[0]);

    // Buttons at the bottom
    let button_layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Length(10), Constraint::Length(10), Constraint::Min(0)])
        .split(chunks[1]);

    let confirm_btn = Paragraph::new("confirm")
        .block(Block::default().borders(Borders::ALL))
        .style(ratatui::style::Style::default().bg(ratatui::style::Color::Blue).fg(ratatui::style::Color::White))
        .alignment(Alignment::Center);

    let back_btn = Paragraph::new("back")
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

    f.render_widget(confirm_btn, button_layout[0]);
    f.render_widget(back_btn, button_layout[1]);
}