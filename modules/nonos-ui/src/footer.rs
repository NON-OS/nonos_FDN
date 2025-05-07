use tui::{Frame, layout::Rect, widgets::{Paragraph, Block, Borders}, style::{Style, Modifier, Color}};

pub fn draw_footer<B: tui::backend::Backend>(f: &mut Frame<B>, area: Rect) {
    let footer = Paragraph::new("Shortcuts: [q] Quit")
        .style(Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, area);
}
