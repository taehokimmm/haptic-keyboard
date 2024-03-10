use std::time::Instant;

pub enum AppState {
    Idle,
    Input,
}

pub struct App {
    pub state: AppState,
    pub input: String,
    pub cpm: u32,
    pub latency: u32, // in nanoseconds
    pub input_rate: u32,
    start: Instant,
    last: Instant,
    count: u32,
}

impl App {
    pub fn new() -> App {
        App {
            state: AppState::Idle,
            input: String::new(),
            cpm: 0,
            latency: 0,
            input_rate: 0,
            start: Instant::now(),
            last: Instant::now(),
            count: 0,
        }
    }

    pub fn handle_input(&mut self, input: char) {
        match self.state {
            AppState::Idle => {}
            AppState::Input => {
                self.input.push(input);
                let now = Instant::now();
                self.input_rate = now.duration_since(self.last).as_millis() as u32;
                self.count += 1;
                self.cpm = match now.duration_since(self.start).as_secs() as u32 {
                    0 => {
                        // Avoid division by zero, and calculate CPM with milliseconds
                        (self.count * 60 * 1000) / now.duration_since(self.start).as_millis() as u32
                    }
                    s => (self.count * 60) / s,
                };
                self.last = now;
            }
        }
    }

    pub fn handle_backspace(&mut self) {
        match self.state {
            AppState::Idle => {}
            AppState::Input => {
                self.input.pop();
            }
        }
    }
    // Enter starts the input mode, and clears the input if already in input mode
    pub fn handle_enter(&mut self) {
        self.start = Instant::now();
        self.count = 0;
        match self.state {
            AppState::Idle => {
                self.state = AppState::Input;
            }
            AppState::Input => {
                self.input.clear();
            }
        }
    }
    // Escape puts the mode back to idle and clears the input. If the mode is already idle, it exits the program
    pub fn handle_escape(&mut self) -> Option<bool> {
        match self.state {
            AppState::Idle => {
                Some(true)
            }
            AppState::Input => {
                self.state = AppState::Idle;
                self.input.clear();
                None
            }
        }
    }
}
