use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};
use crate::app::App;
use crate::ui::theme::DRACULA;
use crate::ui::widgets;
pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = widgets::draw_layout(f, "SELECT EFI PARTITION");

    let rows = app.disks.iter().filter(|d| d.is_efi).map(|disk| {
        Row::new(vec![
            Cell::from(format!("▶ {}\n  contents: {}", disk.name, disk.contents.as_deref().unwrap_or("empty"))),
            Cell::from(format!("{}\n", disk.size)), // Empty bottom line
            Cell::from(format!("{:?}\n", disk.fstype)).style(ratatui::style::Style::default().fg(DRACULA.success)),
        ]).height(2)
    });

    let table = Table::new(rows, [
        Constraint::Percentage(70),
        Constraint::Length(10),
        Constraint::Length(10),
    ])
        .block(Block::default().title("choose the EFI system partition (vfat, ~512MB)"))
        .row_highlight_style(DRACULA.highlight);

    f.render_stateful_widget(table, chunks[1], &mut app.table_state);
}