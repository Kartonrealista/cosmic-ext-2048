use crate::app::*;
use cosmic::{
    iced::{Border, Color},
    iced_core::Shadow,
};

const GREY1RGB: Color = Color {
    r: 238.0 / 255.0,
    g: 228.0 / 255.0,
    b: 218.0 / 255.0,
    a: 1.0,
};

const GREY2RGB: Color = Color {
    r: 237.0 / 255.0,
    g: 224.0 / 255.0,
    b: 200.0 / 255.0,
    a: 1.0,
};

const ORANGE1RGB: Color = Color {
    r: 242.0 / 255.0,
    g: 177.0 / 255.0,
    b: 121.0 / 255.0,
    a: 1.0,
};

const ORANGE2RGB: Color = Color {
    r: 245.0 / 255.0,
    g: 149.0 / 255.0,
    b: 99.0 / 255.0,
    a: 1.0,
};

const RED1RGB: Color = Color {
    r: 246.0 / 255.0,
    g: 124.0 / 255.0,
    b: 96.0 / 255.0,
    a: 1.0,
};

const RED2RGB: Color = Color {
    r: 246.0 / 255.0,
    g: 94.0 / 255.0,
    b: 59.0 / 255.0,
    a: 1.0,
};

pub fn blacktheme(theme: &Theme) -> widget::container::Appearance {
    let mut appearance = orange1theme(theme);
    appearance.icon_color = Some(Color::BLACK);
    appearance.background = Some(cosmic::iced::Background::Color(Color::BLACK));
    appearance
}

pub fn secondary_with_rounder_corners(theme: &Theme) -> widget::container::Appearance {
    let cosmic = theme.cosmic();
    let mut appearance = theme::Container::secondary(&cosmic);
    appearance.border = Border {
        color: Color::TRANSPARENT,
        width: 1.0,
        radius: cosmic.corner_radii.radius_xs.into(),
    };
    appearance
}

pub fn orange1theme(theme: &Theme) -> widget::container::Appearance {
    let cosmic = theme.cosmic();
    widget::container::Appearance {
        icon_color: Some(ORANGE1RGB),
        text_color: Some(Color::WHITE),
        background: Some(cosmic::iced::Background::Color(ORANGE1RGB)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 1.0,
            radius: cosmic.corner_radii.radius_xs.into(),
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            offset: cosmic::iced::Vector::new(0.0, 0.0),
            blur_radius: 0.0,
        },
    }
}

pub fn orange2theme(theme: &Theme) -> widget::container::Appearance {
    let mut appearance = orange1theme(theme);
    appearance.icon_color = Some(ORANGE2RGB);
    appearance.background = Some(cosmic::iced::Background::Color(ORANGE2RGB));
    appearance
}

pub fn gray1theme(theme: &Theme) -> widget::container::Appearance {
    let mut appearance = orange1theme(theme);
    appearance.icon_color = Some(GREY1RGB);
    appearance.background = Some(cosmic::iced::Background::Color(GREY1RGB));
    appearance.text_color = Some(Color {
        r: 119.0 / 255.0,
        g: 110.0 / 255.0,
        b: 101.0 / 255.0,
        a: 1.0,
    });
    appearance
}

pub fn gray2theme(theme: &Theme) -> widget::container::Appearance {
    let mut appearance = gray1theme(theme);
    appearance.icon_color = Some(GREY2RGB);
    appearance.background = Some(cosmic::iced::Background::Color(GREY2RGB));
    appearance
}

pub fn red1theme(theme: &Theme) -> widget::container::Appearance {
    let mut appearance = orange1theme(theme);
    appearance.icon_color = Some(RED1RGB);
    appearance.background = Some(cosmic::iced::Background::Color(RED1RGB));
    appearance
}

pub fn red2theme(theme: &Theme) -> widget::container::Appearance {
    let mut appearance = orange1theme(theme);
    appearance.icon_color = Some(RED2RGB);
    appearance.background = Some(cosmic::iced::Background::Color(RED2RGB));
    appearance
}