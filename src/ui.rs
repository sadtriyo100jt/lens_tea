mod components;

use crate::{app::App, app::Window};
use components::{options, preview, results, search, text_area};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::ListState,
    Frame,
};

struct Colors {
    search: Color,
    options: Color,
}

impl Colors {
    fn new(chosen_window: &Window) -> Self {
        Self {
            search: if chosen_window == &Window::Search {
                Color::Blue
            } else {
                Color::Cyan
            },
            options: if chosen_window == &Window::Options {
                Color::Blue
            } else {
                Color::Cyan
            },
        }
    }
}

pub fn render(app: &mut App, frame: &mut Frame) {
    let colors = Colors::new(&app.window);
    let mut result_state = ListState::default();
    result_state.select(Some(app.scroll.result));

    let mut options_state = ListState::default();
    options_state.select(Some(app.scroll.options));

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15), // Options column
            Constraint::Percentage(50), // Text area, search, and results column
            Constraint::Percentage(35), // Preview column
        ])
        .split(frame.size());

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Text area and search row
            Constraint::Min(0),    // Results row
        ])
        .split(columns[1]);

    frame.render_stateful_widget(options(colors.options), columns[0], &mut options_state);
    frame.render_stateful_widget(results(app), rows[1], &mut result_state);
    frame.render_widget(text_area(app).widget(), rows[0]);
    frame.render_widget(search(colors.search), rows[0]);
    frame.render_widget(preview(app), columns[2]);
}
