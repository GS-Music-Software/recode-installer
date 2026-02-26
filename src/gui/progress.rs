use iced::widget::{column, container, progress_bar, text, Space};
use iced::{Alignment, Element, Length};
use super::{colors, style};
use crate::consts;

pub fn view<'a, Msg: 'a>(
    status: &'a str,
    pct: f32,
) -> Element<'a, Msg> {
    let content = column![
        text(format!("Installing {}", consts::APP_NAME)).size(24),
        Space::with_height(16),
        text(status)
            .size(14)
            .color(colors::DIM),
        Space::with_height(12),
        progress_bar(0.0..=100.0, pct)
            .style(style::bar)
            .height(8),
        Space::with_height(8),
        text(format!("{}%", pct as u32))
            .size(13)
            .color(colors::DIM),
        Space::with_height(Length::Fill),
    ]
    .spacing(4)
    .padding(32)
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::Start);

    container(content)
        .style(style::page)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
