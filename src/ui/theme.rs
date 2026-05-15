use ratatui::style::{Color, Style, Modifier};

pub struct Theme {
    pub base: Style,
    pub border: Style,
    pub title: Style,
    pub highlight: Style,
    pub success: Color,
    pub error: Color,
    pub warning: Color,
    pub titleColor: Color,
}

pub const DRACULA: Theme = Theme {
    base: Style::new().fg(Color::Rgb(248, 248, 242)).bg(Color::Rgb(40, 42, 54)),
    border: Style::new().fg(Color::Rgb(98, 114, 164)),
    title: Style::new().fg(Color::Rgb(189, 147, 249)).add_modifier(Modifier::BOLD),
    titleColor: Color::Rgb(189, 147, 249),
    highlight: Style::new().bg(Color::Rgb(68, 71, 90)).fg(Color::Rgb(255, 121, 198)),
    success: Color::Rgb(80, 250, 123),
    error: Color::Rgb(255, 85, 85),
    warning: Color::Rgb(241, 250, 140),
};
