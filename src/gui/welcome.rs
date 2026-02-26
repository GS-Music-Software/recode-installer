use iced::widget::{button, column, container, text, Space};
use iced::{Alignment, Element, Length};
use super::{colors, style};
use crate::consts;

pub fn view<'a, Msg: Clone + 'a>(
    version: &str,
    on_next: Msg,
    on_cancel: Msg,
) -> Element<'a, Msg> {
    let content = column![
        text(format!("Welcome to {} Setup", consts::APP_NAME)).size(24),
        Space::with_height(12),
        text(format!("This will install {} {} on your computer.", consts::APP_NAME, version))
            .size(14)
            .color(colors::DIM),
        Space::with_height(8),
        text("Click Next to continue.")
            .size(14)
            .color(colors::DIM),
        Space::with_height(Length::Fill),
        iced::widget::row![
            button(text("Cancel").size(14))
                .style(style::btn_secondary)
                .padding([8, 20])
                .on_press(on_cancel),
            Space::with_width(Length::Fill),
            button(text("Next >").size(14))
                .style(style::btn_primary)
                .padding([8, 20])
                .on_press(on_next),
        ]
        .width(Length::Fill),
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
