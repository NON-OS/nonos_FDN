use tui::layout::{Constraint, Direction, Layout, Rect};

/// Splits the screen into 3 vertical sections:
/// [0] Header (Banner)
/// [1] Main Area (Relay, System, ZK etc.)
/// [2] Footer
pub fn layout_chunks(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),   // Banner/Header
            Constraint::Min(10),     // Main content (auto expand)
            Constraint::Length(3),   // Footer
        ])
        .split(area)
}
