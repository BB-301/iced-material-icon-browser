#[derive(Debug, Default, Copy, Clone)]
pub enum CustomTheme {
    #[default]
    Dark,
    Light,
}

impl CustomTheme {
    fn background_color(&self) -> iced::Color {
        match self {
            CustomTheme::Dark => iced::Color::from_rgba8(0x00, 0x00, 0x00, 1.0),
            CustomTheme::Light => iced::Color::from_rgba8(0xff, 0xff, 0xff, 1.0),
        }
    }

    fn text_color(&self) -> iced::Color {
        match self {
            CustomTheme::Dark => iced::Color::from_rgba8(0xff, 0xff, 0xff, 1.0),
            CustomTheme::Light => iced::Color::from_rgba8(0x00, 0x00, 0x00, 1.0),
        }
    }

    fn primary_color(&self) -> iced::Color {
        match self {
            CustomTheme::Dark => iced::Color::from_rgba8(10, 132, 255, 1.0),
            CustomTheme::Light => iced::Color::from_rgba8(0, 122, 255, 1.0),
        }
    }

    fn success_color(&self) -> iced::Color {
        match self {
            CustomTheme::Dark => iced::Color::from_rgba8(48, 209, 81, 1.0),
            CustomTheme::Light => iced::Color::from_rgba8(52, 199, 89, 1.0),
        }
    }

    fn danger_color(&self) -> iced::Color {
        match self {
            CustomTheme::Dark => iced::Color::from_rgba8(255, 69, 58, 1.0),
            CustomTheme::Light => iced::Color::from_rgba8(255, 59, 48, 1.0),
        }
    }

    pub fn to_theme(&self) -> iced::theme::Theme {
        iced::theme::Theme::custom(iced::theme::Palette {
            // [iced example](https://github.com/iced-rs/iced/blob/master/examples/styling/src/main.rs)
            // [apple color guidelines](https://developer.apple.com/design/human-interface-guidelines/color)
            background: self.background_color(),
            text: self.text_color(),
            primary: self.primary_color(),
            success: self.success_color(),
            danger: self.danger_color(),
        })
    }

    pub fn toggle(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

#[derive(Default)]
pub enum ToolbarButtonStyle {
    #[default]
    Default,
    Destructive,
    Text,
}
pub struct ToolbarButton(ToolbarButtonStyle);

impl Default for ToolbarButton {
    fn default() -> Self {
        Self(ToolbarButtonStyle::Default)
    }
}

impl ToolbarButton {
    pub fn destructive() -> Self {
        Self(ToolbarButtonStyle::Destructive)
    }

    pub fn text() -> Self {
        Self(ToolbarButtonStyle::Text)
    }
}

impl std::convert::From<ToolbarButton> for iced::theme::Button {
    fn from(value: ToolbarButton) -> Self {
        iced::theme::Button::Custom(Box::new(value))
    }
}

impl iced::widget::button::StyleSheet for ToolbarButton {
    type Style = iced::theme::Theme;

    fn active(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let color = match self.0 {
            ToolbarButtonStyle::Default => style.extended_palette().primary.base.color,
            ToolbarButtonStyle::Text => style.palette().text,
            ToolbarButtonStyle::Destructive => style.extended_palette().danger.base.color,
        };

        iced::widget::button::Appearance {
            text_color: color,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub enum RowButtonStyle {
    #[default]
    Default,
    Selected,
}
pub struct RowButton(RowButtonStyle);

impl Default for RowButton {
    fn default() -> Self {
        Self(RowButtonStyle::Default)
    }
}

impl RowButton {
    pub fn new(selected: bool) -> Self {
        if selected {
            Self::selected()
        } else {
            Self::default()
        }
    }

    pub fn selected() -> Self {
        Self(RowButtonStyle::Selected)
    }
}

impl std::convert::From<RowButton> for iced::theme::Button {
    fn from(value: RowButton) -> Self {
        iced::theme::Button::Custom(Box::new(value))
    }
}

impl iced::widget::button::StyleSheet for RowButton {
    type Style = iced::theme::Theme;

    fn active(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let background_color = match self.0 {
            RowButtonStyle::Default => iced::Color::TRANSPARENT,
            RowButtonStyle::Selected => style.extended_palette().primary.base.color,
        };

        let border_color = match self.0 {
            RowButtonStyle::Default => iced::Color::TRANSPARENT,
            RowButtonStyle::Selected => style.extended_palette().primary.weak.color,
        };

        let border_width = match self.0 {
            RowButtonStyle::Default => 0.0,
            RowButtonStyle::Selected => 1.0,
        };

        iced::widget::button::Appearance {
            text_color: style.palette().text,
            background: Some(background_color.into()),
            border_color,
            border_width,
            border_radius: 6.0.into(),
            ..Default::default()
        }
    }
}
