//! UI Part of the application:
//! The module handles the Interface and the keyboard input part of the app

use ratatui::layout::Alignment::Center;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::BorderType::Rounded;
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::{App, AppState};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(2),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title = Span::styled(
        "LLHF: Low Latency Haptic Feedback (v0.0.1)",
        Style::default().fg(Color::Blue),
    );
    f.render_widget(
        Paragraph::new(title).block(Block::default().borders(Borders::ALL)),
        chunks[0],
    );

    let input_title = match app.state {
        AppState::Idle => "Press any key to start input, press Esc to quit",
        AppState::Input => "Press Esc to clear input",
    };

    let input = Paragraph::new(app.input.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(input_title)
                .border_type(Rounded)
                .title_alignment(Center)
                .padding(Padding::horizontal(1)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(input, chunks[1]);

    let mode = match app.state {
        AppState::Idle => Paragraph::new("Idle")
            .block(Block::default().borders(Borders::ALL).border_type(Rounded))
            .alignment(Center),
        AppState::Input => Paragraph::new("Receiving...")
            .block(Block::default().borders(Borders::ALL).border_type(Rounded))
            .alignment(Center)
            .style(Style::default().fg(Color::Green)),
    };

    let stats = Paragraph::new(format!(
        "CPM: {} | Latency: {}ns | Input Rate: {}ms",
        app.cpm, app.latency, app.input_rate
    ))
    .block(Block::default().borders(Borders::ALL).title("Stats"));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(18), Constraint::Min(2)])
        .split(chunks[2]);

    f.render_widget(mode, footer_chunks[0]);
    f.render_widget(stats, footer_chunks[1]);
}
