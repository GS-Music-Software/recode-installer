use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Element, Length};
use super::{colors, style};
use crate::consts;

pub fn view<'a, Msg: Clone + 'a>(
    on_reinstall: Msg,
    on_uninstall: Msg,
    on_cancel: Msg,
) -> Element<'a, Msg> {
    let content = column![
        text(format!("{} is already installed.", consts::APP_NAME)).size(24),
        Space::with_height(12),
        text("What would you like to do?")
            .size(14)
            .color(colors::DIM),
        Space::with_height(Length::Fill),
        row![
            button(text("Cancel").size(14))
                .style(style::btn_secondary)
                .padding([8, 20])
                .on_press(on_cancel),
            Space::with_width(Length::Fill),
            button(text("Uninstall").size(14))
                .style(style::btn_secondary)
                .padding([8, 20])
                .on_press(on_uninstall),
            button(text("Reinstall").size(14))
                .style(style::btn_primary)
                .padding([8, 20])
                .on_press(on_reinstall),
        ]
        .spacing(8)
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
