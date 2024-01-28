mod components;

use crate::{app::App, app::Window};
use components::{options, preview, results, search, text_area};
use ratatui::{layout::Rect, style::Color, widgets::ListState, Frame};

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

    frame.render_widget(text_area(app).widget(), Rect::new(27, 0, 90, 5));
    frame.render_widget(search(colors.search), Rect::new(27, 0, 90, 5));

    let mut list_state = ListState::default();
    list_state.select(Some(app.options_scroll));
    frame.render_stateful_widget(
        options(colors.options),
        Rect::new(1, 0, 25, frame.size().height),
        &mut list_state,
    );

    let mut list_state = ListState::default();
    list_state.select(Some(app.result_scroll));
    frame.render_stateful_widget(
        results(app),
        Rect::new(27, 5, 90, frame.size().height - 5),
        &mut list_state,
    );

    frame.render_widget(preview(app), Rect::new(118, 0, 91, frame.size().height));
}
