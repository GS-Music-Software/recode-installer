use iced::widget::{button, container, text_input, progress_bar};
use iced::{Border, Background, Color, Theme};
use super::colors;

pub fn page(theme: &Theme) -> container::Style {
    let _ = theme;
    container::Style {
        background: Some(Background::Color(colors::BG)),
        text_color: Some(colors::WHITE),
        ..Default::default()
    }
}

pub fn btn_primary(theme: &Theme, status: button::Status) -> button::Style {
    let _ = theme;
    let bg = match status {
        button::Status::Hovered => colors::ACCENT_HOVER,
        _ => colors::ACCENT,
    };
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: Color::WHITE,
        border: Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn btn_secondary(theme: &Theme, status: button::Status) -> button::Style {
    let _ = theme;
    let bg = match status {
        button::Status::Hovered => colors::BTN_SEC_HOVER,
        _ => colors::BTN_SEC,
    };
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: colors::DIM,
        border: Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn input(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let _ = theme;
    let border_color = match status {
        text_input::Status::Focused => colors::ACCENT,
        _ => colors::BORDER,
    };
    text_input::Style {
        background: Background::Color(colors::SURFACE),
        border: Border {
            radius: 6.0.into(),
            width: 1.0,
            color: border_color,
        },
        icon: colors::DIM,
        placeholder: colors::DIM,
        value: colors::WHITE,
        selection: colors::ACCENT,
    }
}

pub fn bar(theme: &Theme) -> progress_bar::Style {
    let _ = theme;
    progress_bar::Style {
        background: Background::Color(colors::BTN_SEC),
        bar: Background::Color(colors::ACCENT),
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
    }
}
