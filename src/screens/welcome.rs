use ratatui::{
    layout::{Alignment, Constraint, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use crate::app::App;
use crate::ui::theme::DRACULA;

pub fn render(f: &mut Frame, _app: &mut App) {
    let area = f.size();

    // Split the screen into a Top Banner area and a Main Content area
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Exact height for a bordered single-line header
            Constraint::Min(0),    // Remaining space for the instructions list
        ])
        .split(area);

    // 1. Welcome Header Block with its own borders
    let welcome_block = Block::default()
        .borders(Borders::ALL)
        .border_style(DRACULA.border)
        .style(DRACULA.base);

    let welcome_paragraph = Paragraph::new("Welcome to the System Recovery Environment")
        .block(welcome_block)
        .alignment(Alignment::Center);

    // 2. Main Content Block with its own borders and title
    let main_block = Block::default()
        .borders(Borders::ALL)
        .title(" GRUB RESCUE / INSTALLER ")
        .title_alignment(Alignment::Center)
        .border_style(DRACULA.border)
        .style(DRACULA.base);

    let text = vec![
        "",
        " This tool will help you:",
        "   1. Identify Root/EFI partitions.",
        "   2. Repair FSTAB entries.",
        "   3. Reinstall or update GRUB.",
        "   4. Fix EFI partitions Windows. (Soon)",
        "   5. Resize and move partitions. (Soon)",
        "",
        " Press <Enter> to scan for block devices.",
    ].join("\n");

    let main_paragraph = Paragraph::new(text)
        .block(main_block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    // Render each widget into its respective layout chunk
    f.render_widget(welcome_paragraph, chunks[0]);
    f.render_widget(main_paragraph, chunks[1]);
}