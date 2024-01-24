use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!("",))
            .block(
                Block::default()
                    .title("Full Command")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .alignment(Alignment::Center),
        Rect::new(0, 0, frame.size().width, 5),
    );

    frame.render_widget(
        Paragraph::new(format!("",))
            .block(
                Block::default()
                    .title("Options")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .alignment(Alignment::Center),
        Rect::new(0, 5, 50, frame.size().height - 5),
    );

    frame.render_widget(
        Paragraph::new(format!("",))
            .block(
                Block::default()
                    .title("Output")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .alignment(Alignment::Center),
        Rect::new(50, 5, frame.size().width - 50, frame.size().height - 5),
    );
}
