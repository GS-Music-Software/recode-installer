use iced::widget::{button, column, container, text, Space};
use iced::{Alignment, Element, Length};
use super::{colors, style};

pub fn view<'a, Msg: Clone + 'a>(
    title: &str,
    subtitle: &str,
    on_close: Msg,
) -> Element<'a, Msg> {
    let content = column![
        text(title.to_string()).size(24),
        Space::with_height(12),
        text(subtitle.to_string())
            .size(14)
            .color(colors::DIM),
        Space::with_height(Length::Fill),
        iced::widget::row![
            Space::with_width(Length::Fill),
            button(text("Finish").size(14))
                .style(style::btn_primary)
                .padding([8, 24])
                .on_press(on_close),
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
