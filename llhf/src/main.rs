//! LLHF: Low Latency Haptic Feedback
//!
//! This is an implementation of a low Latency haptic feedback system.
//! The system is designed to give haptic feedback according to the user's keyboard input.
//! For simplicity, we will assume that the user is only pressing one key at a time.
//! The system will give a short feedback when the user presses a key.

use std::io::{stdout, Result};
use std::time::Instant;

use crossterm::{
    event::{self, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use rodio::{OutputStream, Source};

mod app;
mod ui;

mod audio;

pub fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let wav = audio::load_sources();

    let mut app = app::App::new();

    loop {
        terminal.draw(|f| ui::ui(f, &mut app))?; // Draw the UI

        if let event::Event::Key(event) = event::read()? {
            let process_start = Instant::now();
            match event.code {
                KeyCode::Char(c) => {
                    app.handle_input(c);
                    if c.is_ascii_alphabetic() {
                        match wav.get(&c.to_ascii_uppercase()) {
                            Some(source) => {
                                stream_handle
                                    .play_raw(source.clone().convert_samples())
                                    .expect("TODO: panic message");
                                app.latency = process_start.elapsed().as_nanos() as u32;
                            }
                            None => {
                                panic!("No sound for this key: {}", c)
                            }
                        }
                    }
                }
                KeyCode::Backspace => app.handle_backspace(),
                KeyCode::Enter => app.handle_enter(),
                KeyCode::Esc => match app.handle_escape() {
                    Some(true) => break,
                    _ => { /* do nothing */ }
                },
                _ => {}
            }
        }
    }
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
