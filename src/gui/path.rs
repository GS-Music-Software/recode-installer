use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Alignment, Element, Length};
use super::{colors, style};
use crate::consts;

pub fn view<'a, Msg: Clone + 'a>(
    path: &str,
    on_path_change: impl Fn(String) -> Msg + 'a,
    on_browse: Msg,
    on_back: Msg,
    on_install: Msg,
) -> Element<'a, Msg> {
    let content = column![
        text("Choose Install Location").size(24),
        Space::with_height(12),
        text(format!("{} will be installed to the following folder.", consts::APP_NAME))
            .size(14)
            .color(colors::DIM),
        Space::with_height(16),
        row![
            text_input("Install path...", path)
                .on_input(on_path_change)
                .style(style::input)
                .size(14)
                .padding([12, 12])
                .width(Length::Fill),
            button(text("Browse").size(14))
                .style(style::btn_secondary)
                .padding([8, 16])
                .on_press(on_browse),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
        Space::with_height(Length::Fill),
        row![
            button(text("< Back").size(14))
                .style(style::btn_secondary)
                .padding([8, 20])
                .on_press(on_back),
            Space::with_width(Length::Fill),
            button(text("Install").size(14))
                .style(style::btn_primary)
                .padding([8, 20])
                .on_press(on_install),
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
