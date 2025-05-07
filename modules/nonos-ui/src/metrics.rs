use tui::{
    widgets::{Block, Borders, Gauge},
    layout::Rect,
    style::{Color, Modifier, Style},
    Frame,
};
use crate::state::AppState;

pub fn draw_metrics<B: tui::backend::Backend>(f: &mut Frame<B>, area: Rect, state: &AppState) {
    let cpu = Gauge::default()
        .block(Block::default().title("CPU Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Red).bg(Color::Black))
        .percent(state.cpu_usage());
    f.render_widget(cpu, area);
}
