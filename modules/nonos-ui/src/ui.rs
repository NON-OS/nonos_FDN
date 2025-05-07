use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::state::AppState;
use crate::widgets::{
    banner::draw_banner,
    footer::draw_footer,
    metrics::draw_metrics,
};

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(10),    // Banner
            Constraint::Min(1),       // Metrics/Body
            Constraint::Length(1),    // Footer
        ])
        .split(f.size());

    draw_banner(f, chunks[0], app);
    draw_metrics(f, chunks[1], app);
    draw_footer(f, chunks[2]);
}
