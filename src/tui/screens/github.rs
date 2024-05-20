use ratatui::{
    layout::{Alignment, Rect},
    prelude::Buffer,
    style::{Color, Style, Stylize},
    widgets::{block::Title, Block, Borders, Padding, Paragraph, StatefulWidget, Widget},
};

use crate::tui::details::{test, DetailsObject};

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
        let container = Block::default();

        let container_rect = container.inner(area);
        container.render(area, buf);

        Paragraph::new("Yes browser auth yes")
            .bg(Color::Red)
            .underlined()
            .render(container_rect, buf);
    }

    fn render_token(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Paragraph::new("Yes token auth yes")
            .bg(Color::Blue)
            .underlined()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Red)),
            )
            .render(area, buf);
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

        let details = test(
            [
                DetailsObject::new("Browser auth", 10)
                    .opened(matches!(state.selected_mode, GithubAuthMode::Browser)),
                DetailsObject::new("Token auth", 20)
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

// fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
//     let popup_layout = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([
//             Constraint::Percentage((100 - percent_y) / 2),
//             Constraint::Percentage(percent_y),
//             Constraint::Percentage((100 - percent_y) / 2),
//         ])
//         .split(r);
//
//     Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([
//             Constraint::Percentage((100 - percent_x) / 2),
//             Constraint::Percentage(percent_x),
//             Constraint::Percentage((100 - percent_x) / 2),
//         ])
//         .split(popup_layout[1])[1]
// }
