use tui::{widgets::*, style::*, text::*, layout::Rect, backend::Backend, Frame};

use crate::data::AppState;

pub fn draw_banner<B: Backend>(f: &mut Frame<B>, area: Rect, _app: &AppState) {
    let block = Block::default()
        .title(Span::styled(
            "NONOS ZeroState UI",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL);

    f.render_widget(block, area);
}
