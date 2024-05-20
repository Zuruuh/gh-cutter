use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    prelude::Buffer,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{
        block::Title, Block, BorderType, Borders, Padding, Paragraph, StatefulWidget, Widget,
    },
};

#[derive(Default)]
pub struct GithubScreen;

#[derive(Default)]
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

impl StatefulWidget for GithubScreen {
    type State = GithubScreenState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut Buffer, state: &mut Self::State) {
        let container = Block::default()
            .padding(Padding::new(1, 1, 1, 1))
            .title(Title::from("GitHub Login".bold()).alignment(Alignment::Center));

        let container_rect = container.inner(area);

        let constraints = match state.selected_mode {
            GithubAuthMode::Browser => [
                Constraint::Percentage(60),
                Constraint::Length(4),
                Constraint::Fill(1),
            ],
            GithubAuthMode::Token => [
                Constraint::Fill(1),
                Constraint::Length(4),
                Constraint::Percentage(60),
            ],
        };

        let [browser_mode_rect, or, token_mode_rect] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(container_rect)[..]
        else {
            unreachable!()
        };

        Paragraph::new(" OR ")
            .block(Block::default().padding(Padding::top(container_rect.as_size().height / 2)))
            .render(or, buf);

        let selected_mode_container_border_style = Style::new().fg(Color::LightGreen);
        let not_selected_mode_container_border_style = Style::new().fg(Color::DarkGray);

        let mut browser_mode_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(Title::from("Browser auth".bold()).alignment(Alignment::Center));

        let mut token_mode_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(Title::from("Token auth".bold()).alignment(Alignment::Center));

        match state.selected_mode {
            GithubAuthMode::Browser => {
                browser_mode_block =
                    browser_mode_block.border_style(selected_mode_container_border_style);

                let [browser_mode_subtitle, _, browser_mode_button] = Layout::new(
                    Direction::Vertical,
                    [
                        Constraint::Percentage(5),
                        Constraint::Fill(1),
                        Constraint::Percentage(15),
                    ],
                )
                .split(browser_mode_rect)[..] else {
                    unreachable!()
                };

                token_mode_block =
                    token_mode_block.border_style(not_selected_mode_container_border_style);

                Line::from("Focus to expand.")
                    .centered()
                    .dark_gray()
                    .render(centered_rect(token_mode_rect, 100, 1), buf);
            }
            GithubAuthMode::Token => {
                browser_mode_block =
                    browser_mode_block.border_style(not_selected_mode_container_border_style);

                Line::from("Focus to expand.")
                    .centered()
                    .dark_gray()
                    .render(centered_rect(browser_mode_rect, 100, 1), buf);

                token_mode_block =
                    token_mode_block.border_style(selected_mode_container_border_style);
            }
        };

        browser_mode_block.render(browser_mode_rect, buf);
        token_mode_block.render(token_mode_rect, buf);

        container.render(area, buf);
    }
}

fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
