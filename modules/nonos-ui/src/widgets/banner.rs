use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::AppState;

pub fn draw_banner<B: Backend>(f: &mut Frame<B>, area: Rect, _state: &AppState) {
    let banner = vec![
        Spans::from(Span::styled("███╗   ██╗ ██████╗ ███╗   ██╗ ██████╗ ███████╗", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Spans::from(Span::styled("████╗  ██║██╔═══██╗████╗  ██║██╔═══██╗██╔════╝", Style::default().fg(Color::Cyan))),
        Spans::from(Span::styled("██╔██╗ ██║██║   ██║██╔██╗ ██║██║   ██║███████╗", Style::default().fg(Color::Cyan))),
        Spans::from(Span::styled("██║╚██╗██║██║   ██║██║╚██╗██║██║   ██║╚════██║", Style::default().fg(Color::Cyan))),
        Spans::from(Span::styled("██║ ╚████║╚██████╔╝██║ ╚████║╚██████╔╝███████║", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Spans::from(Span::styled("╚═╝  ╚═══╝ ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝ ╚══════╝", Style::default().fg(Color::Cyan))),
        Spans::from(Span::styled(" ", Style::default())),
        Spans::from(Span::styled("Welcome to NONOS", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))),
        Spans::from(Span::styled("The Decentralized Operating System", Style::default().fg(Color::Magenta).add_modifier(Modifier::ITALIC))),
    ];

    let paragraph = Paragraph::new(banner)
        .block(Block::default()
            .title("NONOS Boot Banner")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)))
        .alignment(tui::layout::Alignment::Center);

    f.render_widget(paragraph, area);
}
