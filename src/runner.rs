use anyhow::Result;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::{
    io::{Stdout, stdout},
    time::Duration,
};

use crate::app::App;

struct Runner {
    app: App,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Runner {
    pub fn new() -> Result<Self> {
        let app = App::new();

        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;
        Ok(Self { app, terminal })
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;

        loop {
            if crossterm::event::poll(Duration::from_millis(16))? {
                let event = crossterm::event::read()?;
                self.app.handle_event(event);
            }

            self.app.update();

            self.terminal.draw(|frame| self.app.render(frame))?;

            if self.app.should_quit() {
                break;
            }
        }

        disable_raw_mode()?;
        execute!(stdout(), LeaveAlternateScreen)?;
        Ok(())
    }
}
