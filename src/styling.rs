#[derive(Debug, Default, Copy, Clone)]
pub enum CustomTheme {
    #[default]
    Dark,
    Light,
}

impl CustomTheme {
    fn background_color(&self) -> iced::Color {
        match self {
            // CustomTheme::Dark => iced::Color::from_rgba8(0x00, 0x00, 0x00, 1.0),
            CustomTheme::Dark => iced::Color::from_rgba8(30, 30, 30, 1.0),
            CustomTheme::Light => iced::Color::from_rgba8(0xff, 0xff, 0xff, 1.0),
        }
    }

    fn text_color(&self) -> iced::Color {
        match self {
            // CustomTheme::Dark => iced::Color::from_rgba8(0xff, 0xff, 0xff, 1.0),
            CustomTheme::Dark => iced::Color::from_rgba8(221, 221, 221, 1.0),
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
pub enum CustomContainerStyle {
    #[default]
    Default,
    Toolbar,
    Sidebar,
}

pub struct CustomContainer(CustomContainerStyle);

impl Default for CustomContainer {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl CustomContainer {
    pub fn toolbar() -> Self {
        Self(CustomContainerStyle::Toolbar)
    }

    pub fn sidebar() -> Self {
        Self(CustomContainerStyle::Sidebar)
    }

    pub fn move_to_style(self) -> iced::theme::Container {
        self.into()
    }
}

impl std::convert::From<CustomContainer> for iced::theme::Container {
    fn from(value: CustomContainer) -> Self {
        iced::theme::Container::Custom(Box::new(value))
    }
}

impl iced::widget::container::StyleSheet for CustomContainer {
    type Style = iced::theme::Theme;

    fn appearance(&self, style: &Self::Style) -> iced::widget::container::Appearance {
        let background_color = match self.0 {
            CustomContainerStyle::Default => style.palette().background,
            CustomContainerStyle::Toolbar => {
                let mut c = style.palette().background;
                c.r += 0.03;
                c.g += 0.03;
                c.b += 0.03;
                c
            }
            CustomContainerStyle::Sidebar => {
                let mut c = style.palette().background;
                c.r += 0.05;
                c.g += 0.05;
                c.b += 0.05;
                c
            }
        };

        iced::widget::container::Appearance {
            background: Some(background_color.into()),
            border_radius: 0.0.into(),
            border_color: iced::Color::TRANSPARENT,
            border_width: 0.0,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub enum RowButtonStyle {
    #[default]
    Default,
    LightlyBordered,
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

    pub fn new_bordered(selected: bool) -> Self {
        if selected {
            Self::selected()
        } else {
            Self::lightly_bordered()
        }
    }

    pub fn selected() -> Self {
        Self(RowButtonStyle::Selected)
    }

    pub fn lightly_bordered() -> Self {
        Self(RowButtonStyle::LightlyBordered)
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
            RowButtonStyle::LightlyBordered => iced::Color::TRANSPARENT,
            RowButtonStyle::Selected => style.extended_palette().primary.base.color,
        };

        let border_color = match self.0 {
            RowButtonStyle::Default => iced::Color::TRANSPARENT,
            RowButtonStyle::LightlyBordered => iced::Color {
                a: 0.1,
                ..style.palette().text
            },
            RowButtonStyle::Selected => style.extended_palette().primary.weak.color,
        };

        let border_width = match self.0 {
            RowButtonStyle::Default => 0.0,
            RowButtonStyle::LightlyBordered => 1.0,
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

#[derive(Default)]
pub enum CustomRuleStyle {
    #[default]
    Default,
    Dark,
}

pub struct CustomRule(CustomRuleStyle);

impl Default for CustomRule {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl CustomRule {
    pub fn dark() -> Self {
        Self(CustomRuleStyle::Dark)
    }

    pub fn move_to_style(self) -> iced::theme::Rule {
        self.into()
    }
}

impl iced::widget::rule::StyleSheet for CustomRule {
    type Style = iced::theme::Theme;

    fn appearance(&self, style: &Self::Style) -> iced::widget::rule::Appearance {
        iced::widget::rule::Appearance {
            color: match self.0 {
                CustomRuleStyle::Default => iced::Color {
                    a: 0.1,
                    ..style.palette().text
                },
                CustomRuleStyle::Dark => iced::Color {
                    a: 1.0,
                    ..iced::Color::BLACK
                },
            },
            fill_mode: iced::widget::rule::FillMode::Full,
            radius: 0.0.into(),
            width: 2,
        }
    }
}

impl std::convert::From<CustomRule> for iced::theme::Rule {
    fn from(value: CustomRule) -> Self {
        iced::theme::Rule::Custom(Box::new(value))
    }
}
