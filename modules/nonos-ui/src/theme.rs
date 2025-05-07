use tui::style::{Color, Modifier, Style};

pub const PRIMARY: Color = Color::Cyan;

pub struct Theme {
    pub primary: Style,
    pub secondary: Style,
    pub accent: Style,
    pub warning: Style,
    pub error: Style,
    pub highlight: Style,
    pub border: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: Style::default().fg(Color::Cyan),
            secondary: Style::default().fg(Color::Gray),
            accent: Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
            warning: Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            error: Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            highlight: Style::default().fg(Color::Magenta).add_modifier(Modifier::ITALIC),
            border: Style::default().fg(Color::DarkGray),
        }
    }
}
