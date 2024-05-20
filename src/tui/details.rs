use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{block::Title, Block, Borders, Padding, Widget},
};

pub struct DetailsObject {
    title: String,
    height: u16,
    opened: bool,
}

impl DetailsObject {
    pub fn new(title: impl ToString, height: u16) -> Self {
        Self {
            title: title.to_string(),
            opened: false,
            height,
        }
    }

    pub fn opened(self, opened: bool) -> Self {
        Self {
            title: self.title,
            height: self.height,
            opened,
        }
    }
}

pub fn render_details(
    objects: impl IntoIterator<Item = DetailsObject>,
    area: Rect,
    buf: &mut Buffer,
) -> Vec<Option<Rect>> {
    let objects = objects.into_iter().collect::<Vec<_>>();
    let layout = Layout::new(
        Direction::Vertical,
        objects.iter().map(|object| {
            object
                .opened
                .then(|| Constraint::Length(object.height))
                .unwrap_or_else(|| Constraint::Length(1))
        }),
    )
    .split(area);

    let mut inner_rects = Vec::<Option<Rect>>::new();

    let objects_count = objects.len();
    for (i, (object, layout)) in objects.into_iter().zip(layout.into_iter()).enumerate() {
        let is_last_rendered_object = (i + 1) == objects_count;

        let color = object
            .opened
            .then(|| Color::White)
            .unwrap_or_else(|| Color::DarkGray);

        let object_block = Block::default()
            .title(Title::from(object.title.fg(color)).alignment(Alignment::Center))
            .title(
                Title::from(object.opened.then(|| "▼ ").unwrap_or_else(|| "▶ "))
                    .alignment(Alignment::Left),
            )
            .padding(Padding::new(1, 1, 1, 1))
            .borders(
                Borders::TOP
                    | object.opened.then(|| Borders::RIGHT).unwrap_or_default()
                    | is_last_rendered_object
                        .then(|| Borders::BOTTOM)
                        .unwrap_or_default(),
            )
            .border_style(Style::default().fg(color));

        let inner_rect = object_block.inner(layout.clone());
        object_block.render(*layout, buf);

        inner_rects.push(object.opened.then_some(inner_rect));
    }

    inner_rects
}
