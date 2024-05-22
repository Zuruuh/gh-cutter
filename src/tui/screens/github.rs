use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::Buffer,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{
        block::Title, Block, BorderType, Borders, Clear, Padding, Paragraph, StatefulWidget, Widget,
    },
};
use tui_input::{backend::crossterm::EventHandler, Input};

use crate::tui::{
    details::{render_details, DetailsObject},
    KeyAction,
};

#[derive(Default)]
pub struct GithubScreen;

#[derive(Default, PartialEq, Eq)]
pub enum GithubAuthMode {
    #[default]
    Browser,
    Token,
}

#[derive(Default)]
pub struct GithubScreenState {
    pub selected_mode: GithubAuthMode,
    pub modal_opened: bool,
    pub input: Input,
}

impl Widget for GithubAuthMode {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self {
            GithubAuthMode::Browser => self.render_browser(area, buf),
            GithubAuthMode::Token => self.render_token(area, buf),
        }
    }
}

impl GithubAuthMode {
    fn render_browser(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [subtitle_rect, _, button_rect] = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ],
        )
        .split(area)[..] else {
            unreachable!()
        };

        let [_, button_rect, _] = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Fill(1),
                Constraint::Length(10),
                Constraint::Fill(1),
            ],
        )
        .split(button_rect)[..] else {
            unreachable!()
        };

        Paragraph::new("Login from browser")
            .bold()
            .centered()
            .render(subtitle_rect, buf);

        create_select_button().render(button_rect, buf);
    }

    fn render_token(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [subtitle_rect, _, instructions_rect, link_rect, _, button_rect] = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ],
        )
        .split(area)[..] else {
            unreachable!()
        };

        let [_, button_rect, _] = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Fill(1),
                Constraint::Length(10),
                Constraint::Fill(1),
            ],
        )
        .split(button_rect)[..] else {
            unreachable!()
        };

        Paragraph::new("Login with personal access token")
            .bold()
            .centered()
            .render(subtitle_rect, buf);

        Paragraph::new(Line::from(vec![
            Span::styled("Token will need the ", Style::new().dark_gray()),
            Span::styled("repo", Style::new().bold().underlined()),
            Span::styled(" scope", Style::new().dark_gray()),
        ]))
        .centered()
        .render(instructions_rect, buf);

        Paragraph::new(
            Span::from(r"https://github.com/settings/tokens/new?scopes=repo&description=GHCutter")
                .underlined(),
        )
        .centered()
        .dark_gray()
        .render(link_rect, buf);

        create_select_button().render(button_rect, buf);
    }
}

impl StatefulWidget for GithubScreen {
    type State = GithubScreenState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let container = Block::default()
            .padding(Padding::new(1, 1, 1, 1))
            .title(Title::from("\u{f09b} GitHub Login".bold()).alignment(Alignment::Center));

        let [subtitle_rect, container_rect] = Layout::new(
            Direction::Vertical,
            [Constraint::Length(2), Constraint::Fill(1)],
        )
        .split(container.inner(area))[..] else {
            unreachable!()
        };

        container.render(area, buf);

        Paragraph::new(Line::from(vec![
            Span::from("Credentials will be stored at "),
            Span::from("$XDG_CONFIG_HOME/gh-cutter.toml").bold(),
        ]))
        .centered()
        .render(subtitle_rect, buf);

        let details = render_details(
            [
                DetailsObject::new("Browser auth", 8)
                    .opened(matches!(state.selected_mode, GithubAuthMode::Browser)),
                DetailsObject::new("Token auth", 10)
                    .opened(matches!(state.selected_mode, GithubAuthMode::Token)),
            ],
            container_rect,
            buf,
        );

        for (rect, widget) in details
            .into_iter()
            .zip([GithubAuthMode::Browser, GithubAuthMode::Token])
        {
            if let Some(rect) = rect {
                widget.render(rect, buf);
            }
        }

        // Form modal

        if !state.modal_opened {
            return;
        }

        let modal_rect = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Fill(1),
                Constraint::Length(50),
                Constraint::Fill(1),
            ],
        )
        .split(
            Layout::new(
                Direction::Vertical,
                [
                    Constraint::Fill(1),
                    Constraint::Length(15),
                    Constraint::Fill(1),
                ],
            )
            .split(area)[1],
        )[1];

        Clear::default().render(modal_rect, buf);

        let modal = Block::new()
            .borders(Borders::ALL)
            .border_style(Style::new().white())
            .border_type(BorderType::Rounded)
            .padding(Padding::new(1, 1, 1, 1))
            .title_top("Paste your token")
            .title_bottom(
                Line::from(vec![Span::from("ESC").bold(), Span::from(" to exit")])
                    .alignment(Alignment::Left),
            );

        modal.render(modal_rect, buf);

        let [_, modal_input_rect, _, submit_input_rect, _] = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ],
        )
        .split(modal_rect)[..] else {
            unreachable!()
        };

        let modal_input_rect = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(2),
            ],
        )
        .split(modal_input_rect)[1];

        let (text_before_cursor, text_after_cursor) =
            state.input.value().split_at(state.input.cursor());
        let char_under_cursor = text_after_cursor.chars().next().unwrap_or_else(|| ' ');
        let text_after_cursor = text_after_cursor.chars().skip(1).collect::<String>();

        Paragraph::new(Line::from(vec![
            Span::from(text_before_cursor.to_string()),
            Span::from(char_under_cursor.to_string()).black().on_white(),
            Span::from(text_after_cursor.to_string()),
        ]))
        .block(Block::new().borders(Borders::ALL))
        .render(modal_input_rect, buf);

        let submit_input_rect = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Fill(1),
                Constraint::Length(8),
                Constraint::Fill(1),
            ],
        )
        .split(submit_input_rect)[1];

        create_select_button().render(submit_input_rect, buf);
    }
}

impl GithubScreen {
    pub fn on_key_press(key: KeyEvent, state: &mut GithubScreenState) -> Option<KeyAction> {
        if state.modal_opened {
            match key.code {
                KeyCode::Esc => {
                    state.modal_opened = false;

                    None
                }

                KeyCode::Enter => {
                    // Submit
                    Some(KeyAction::Submit)
                }

                _ => match state.input.handle_event(&Event::Key(key)) {
                    Some(_) => None,
                    None => Some(KeyAction::Submit),
                },
            }
        } else {
            match key.code {
                KeyCode::Down | KeyCode::Up | KeyCode::Char('j') | KeyCode::Char('k') => {
                    state.selected_mode = match state.selected_mode {
                        GithubAuthMode::Browser => GithubAuthMode::Token,
                        GithubAuthMode::Token => GithubAuthMode::Browser,
                    };

                    None
                }
                KeyCode::Enter => {
                    state.modal_opened = true;

                    None
                }
                _ => Some(KeyAction::Bubble),
            }
        }
    }
}

fn create_select_button<'a>() -> Paragraph<'a> {
    Paragraph::new("Select").centered().on_blue().white()
}
