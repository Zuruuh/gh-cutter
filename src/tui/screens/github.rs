use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::Buffer,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{block::Title, Block, Padding, Paragraph, StatefulWidget, Widget},
};

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
    // pub token: String,
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

        Paragraph::new(Span::from(
            r"https://github.com/settings/tokens/new?scopes=repo&description=GH%20Cutter",
        ))
        .centered()
        .dark_gray()
        .render(link_rect, buf);

        create_select_button().render(button_rect, buf)
    }
}

impl StatefulWidget for GithubScreen {
    type State = GithubScreenState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let container = Block::default()
            .padding(Padding::new(1, 1, 1, 1))
            .title(Title::from("GitHub Login".bold()).alignment(Alignment::Center));

        let container_rect = container.inner(area);
        container.render(area, buf);

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
    }
}

impl GithubScreen {
    pub fn on_key_press(key: KeyEvent, state: &mut GithubScreenState) -> Option<KeyAction> {
        Some(match key.code {
            KeyCode::Char('q') => KeyAction::Exit,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => KeyAction::Exit,
            KeyCode::Down | KeyCode::Up | KeyCode::Char('j') | KeyCode::Char('k') => {
                if state.modal_opened {
                    return None;
                }

                state.selected_mode = match state.selected_mode {
                    GithubAuthMode::Browser => GithubAuthMode::Token,
                    GithubAuthMode::Token => GithubAuthMode::Browser,
                };

                return None;
            }
            KeyCode::Esc => {
                if state.modal_opened {
                    state.modal_opened = false;

                    return None;
                }

                KeyAction::Exit
            }
            KeyCode::Enter => {
                if !state.modal_opened {
                    state.modal_opened = true;
                }

                return None;
            }
            _ => return None,
        })
    }
}

fn create_select_button<'a>() -> Paragraph<'a> {
    Paragraph::new("Select").centered().on_blue().white()
}
