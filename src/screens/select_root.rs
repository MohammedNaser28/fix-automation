use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};
use crate::app::App;
use crate::ui::theme::DRACULA;
use crate::ui::widgets;

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = widgets::draw_layout(f, "SELECT ROOT PARTITION");

    let header_cells = ["device", "size", "fstype", "label / uuid"]
        .iter()
        .map(|h| Cell::from(*h).style(DRACULA.title));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.disks.iter().map(|disk| {
        let fstype_color = match disk.fstype.as_deref().unwrap_or("unknown") {
            "ext4" => DRACULA.success,
            "ntfs" => DRACULA.warning,
            "swap" => DRACULA.titleColor, // Purple
            _ => DRACULA.base.fg.unwrap(),
        };

        Row::new(vec![
            Cell::from(format!("  {}", disk.name)),
            Cell::from(disk.size.clone()),
            Cell::from(disk.fstype.clone().unwrap()).style(ratatui::style::Style::default().fg(fstype_color)),
            Cell::from(disk.label.clone().unwrap_or_else(|| "—".to_string())),
        ]).height(2) // Height 2 gives that "roomy" look in your image
    });

    let table = Table::new(rows, [
        Constraint::Percentage(25),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(45),
    ])
        .header(header)
        .block(Block::default().borders(Borders::NONE))
        .row_highlight_style(DRACULA.highlight)
        .highlight_symbol("▶ ");

    f.render_stateful_widget(table, chunks[1], &mut app.table_state);
}