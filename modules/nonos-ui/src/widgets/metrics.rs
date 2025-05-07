use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use crate::state::AppState;

pub fn draw_metrics<B: tui::backend::Backend>(
    f: &mut Frame<B>,
    area: Rect,
    state: &AppState,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(area);

    let cpu = Gauge::default()
        .block(Block::default().title("CPU Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent((state.ticks % 100) as u16);

    let mem = Gauge::default()
        .block(Block::default().title("Memory Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Magenta))
        .percent((state.ticks * 3 % 100) as u16);

    let net = Paragraph::new(vec![Spans::from(vec![Span::styled(
        format!("Relay Status: {}", state.ticks % 2),
        Style::default().fg(Color::Cyan),
    )])])
    .block(Block::default().title("Network Relay").borders(Borders::ALL));

    f.render_widget(cpu, chunks[0]);
    f.render_widget(mem, chunks[1]);
    f.render_widget(net, chunks[2]);
}
