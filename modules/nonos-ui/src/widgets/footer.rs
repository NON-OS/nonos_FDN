use tui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::theme;

pub fn draw_footer<B: tui::backend::Backend>(f: &mut Frame<B>, area: Rect) {
    let text = Spans::from(vec![
        Span::styled("[Q] Quit", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        Span::styled(
            "[R] Refresh",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        Span::styled(
            "[NØNOS ZeroState]",
            Style::default().fg(theme::PRIMARY).add_modifier(Modifier::ITALIC),
        ),
    ]);

    let footer = Paragraph::new(text)
        .block(Block::default().borders(Borders::TOP).title("Shortcuts"));

    f.render_widget(footer, area);
}
