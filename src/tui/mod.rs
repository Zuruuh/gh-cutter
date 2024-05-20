use crossterm::{
    event::{self, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result, Stdout};

use crate::config::Config;

use self::screens::github::{GithubAuthMode, GithubScreen, GithubScreenState};

mod details;
mod screens;

pub enum KeyAction {
    Exit,
}

pub struct App {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    config: Option<Config>,
}

impl App {
    pub fn init() -> Result<()> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;

        Ok(())
    }

    pub fn restore() -> Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }

    pub fn new(config: Option<Config>) -> Result<Self> {
        Self::init()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        Ok(Self { terminal, config })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut github_screen_state = GithubScreenState::default();

        loop {
            self.terminal.draw(|frame| {
                let area = frame.size();
                frame.render_stateful_widget(
                    GithubScreen::default(),
                    area,
                    &mut github_screen_state,
                );
            })?;

            if event::poll(std::time::Duration::from_millis(16))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match GithubScreen::on_key_press(key, &mut github_screen_state) {
                            Some(action) => match action {
                                KeyAction::Exit => {
                                    break;
                                }
                            },
                            None => {}
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        App::restore().unwrap();
    }
}
