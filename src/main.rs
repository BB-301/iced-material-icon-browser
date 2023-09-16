use iced::Application as _;

use copy_to_clipboard_animation::{AnimationInfo, CopyType};
use models::{LoadError, MaterialFontMeta, MaterialFontMetaList};
use styling::{CustomContainer, CustomRule, CustomTheme, RowButton, ToolbarButton};

mod models;
mod styling;
mod text_input_wrapper;

#[cfg(windows)]
const ICONS_FONT_BYTES: &[u8] = include_bytes!("..\\resources\\MaterialIcons-Regular.ttf");
#[cfg(unix)]
const ICONS_FONT_BYTES: &[u8] = include_bytes!("../resources/MaterialIcons-Regular.ttf");
#[cfg(windows)]
const ICONS_META_BYTES: &[u8] =
    include_bytes!("..\\resources\\2023-09-12-material-icons-meta.json");
#[cfg(unix)]
const ICONS_META_BYTES: &[u8] = include_bytes!("../resources/2023-09-12-material-icons-meta.json");
#[cfg(windows)]
const FONT_BYTES_REGULAR: &[u8] = include_bytes!("..\\resources\\Roboto\\Roboto-Regular.ttf");
#[cfg(unix)]
const FONT_BYTES_REGULAR: &[u8] = include_bytes!("../resources/Roboto/Roboto-Regular.ttf");
#[cfg(windows)]
const FONT_BYTES_BOLD: &[u8] = include_bytes!("..\\resources\\Roboto\\Roboto-Bold.ttf");
#[cfg(unix)]
const FONT_BYTES_BOLD: &[u8] = include_bytes!("../resources/Roboto/Roboto-Bold.ttf");

const APP_TITLE: &'static str = "Iced Material Icon Browser";

const ICONS_FONT_NAME: &'static str = "Material Icons";
const FONT_NAME: &'static str = "Roboto";

const SCROLLABLE_ICON_LIST_ID: &'static str = "scrollable_icon_list_id";
const SEARCH_TEXT_INPUT_ID: &'static str = "search_text_input_id";

const SPACING_SMALL: u16 = 5;
const SPACING_NORMAL: u16 = 10;
const SPACING_LARGE: u16 = 20;
const SPACING_EXTRA_LARGE: u16 = 40;

const FONT_SIZE_SMALLER: f32 = 12.0;
const FONT_SIZE_SMALL: f32 = 13.0;
const FONT_SIZE_STANDARD: f32 = 14.0;
const FONT_SIZE_LARGE: f32 = 15.0;

const ICON_FONT_SIZE_BIG: u16 = 96;
const ICON_FONT_SIZE_SMALL: u16 = 24;
const ICON_FONT_SIZE_SMALLER: u16 = 20;
const ICON_FONT_SIZE_TINY: u16 = 14;
const ICON_FONT_SIZE_MEDIUM: u16 = 34;
const ICON_FONT_SIZE_TOOLBAR: u16 = 24;

const SIDEBAR_WIDTH: f32 = 200.0;

const WINDOW_INITIAL_WIDTH: u32 = 1000;
const WINDOW_INITIAL_HEIGHT: u32 = 600;

const WINDOW_MIN_WIDTH: u32 = 800;
const WINDOW_MIN_HEIGHT: u32 = 450;

const COPY_ANIMATION_TICKS_MS: u64 = 10;
const COPY_ANIMATION_STEPS: u64 = 100;

const CODEPOINT_COPY: u32 = 57677;
const CODEPOINT_SUCCESS: u32 = 59693;
const CODEPOINT_GRID: u32 = 59824;
const CODEPOINT_LIST: u32 = 57921;
const CODEPOINT_SEARCH: u32 = 59574;
const CODEPOINT_CLOSE: u32 = 58829;
const CODEPOINT_CLOSE_CIRCLE: u32 = 58825;

fn capitalized_string(s: impl Into<String>) -> String {
    let s: String = s.into();
    if s.is_empty() {
        return s;
    }
    let c = format!("{}", &s[0..1]).to_ascii_uppercase();
    format!("{}{}", c, &s[1..])
}

async fn type_to_async<T>(t: T) -> T {
    t
}

fn main() -> iced::Result {
    MyApp::run(iced::Settings {
        window: iced::window::Settings {
            position: iced::window::Position::Specific(20, 900),
            size: (WINDOW_INITIAL_WIDTH, WINDOW_INITIAL_HEIGHT),
            min_size: Some((WINDOW_MIN_WIDTH, WINDOW_MIN_HEIGHT)),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Clone, Debug)]
struct MyApp {
    meta_list: MaterialFontMetaList,
    loaded_resources_count: usize,
    selected_category: Option<String>,
    search_text: String,
    search_visible: bool,
    codepoint: Option<u32>,
    custom_theme: CustomTheme,
    grid_view: bool,
    window_size: (u32, u32),
    copy_animation_info: Option<AnimationInfo>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            meta_list: Default::default(),
            loaded_resources_count: 0,
            selected_category: None,
            search_text: String::new(),
            search_visible: false,
            codepoint: None,
            custom_theme: CustomTheme::new(),
            grid_view: true,
            window_size: (WINDOW_INITIAL_WIDTH, WINDOW_INITIAL_HEIGHT),
            copy_animation_info: None,
        }
    }
}

#[derive(Clone, Debug)]
enum MyMessage {
    FontLoaded,
    MetaListLoaded(MaterialFontMetaList),
    Category(Option<String>),
    Search(String),
    SearchFocusState(bool),
    SearchVisibleState(bool),
    GridViewState(bool),
    Codepoint(Option<u32>),
    Event(iced::event::Event),
    Copy(String, AnimationInfo),
    CopiedAnimationTick,
}

impl MyApp {
    fn icons_font(&self) -> iced::Font {
        iced::Font {
            weight: iced::font::Weight::Normal,
            family: iced::font::Family::Name(ICONS_FONT_NAME),
            monospaced: false,
            stretch: iced::font::Stretch::Normal,
        }
    }

    fn font(&self) -> iced::Font {
        iced::Font {
            weight: iced::font::Weight::Normal,
            family: iced::font::Family::Name(FONT_NAME),
            monospaced: true,
            stretch: iced::font::Stretch::Normal,
        }
    }

    fn bold_font(&self) -> iced::Font {
        iced::Font {
            weight: iced::font::Weight::Bold,
            ..self.font()
        }
    }

    fn selected_font(&self, selected: bool) -> iced::Font {
        if selected {
            self.bold_font()
        } else {
            self.font()
        }
    }

    fn are_resources_loaded(&self) -> bool {
        self.loaded_resources_count == 4
    }

    fn view_sidebar(&self) -> iced::Element<'_, MyMessage> {
        let searching: bool = !self.search_text.is_empty();

        let mut categories = self
            .meta_list
            .categories()
            .iter()
            .zip(self.meta_list.category_codepoints())
            .map(|(name, category_codepoint)| {
                let selected = if let Some(current) = self.selected_category.as_ref() {
                    name == current
                } else {
                    false
                };
                let text = iced::widget::text(capitalized_string(name))
                    .font(self.selected_font(selected && !searching))
                    .size(FONT_SIZE_STANDARD);
                let icon = {
                    let icon = iced::widget::text(char::from_u32(*category_codepoint).unwrap())
                        .font(self.icons_font())
                        .size(ICON_FONT_SIZE_TINY);
                    if !selected {
                        icon.style(iced::theme::Text::Color(self.theme().palette().primary))
                    } else {
                        icon
                    }
                };
                iced::widget::button(
                    iced::widget::row!(icon, text)
                        .align_items(iced::Alignment::Center)
                        .spacing(SPACING_NORMAL),
                )
                .on_press(MyMessage::Category(Some(name.clone())))
                .style(RowButton::new(selected && !searching).into())
                .width(iced::Length::Fill)
                .into()
            })
            .collect::<Vec<iced::Element<'_, MyMessage>>>();

        let all = {
            let selected = self.selected_category.is_none() || searching;
            let text = iced::widget::text("All")
                .font(self.selected_font(selected))
                .size(FONT_SIZE_STANDARD);
            let icon = {
                let icon = iced::widget::text(char::from_u32(CODEPOINT_GRID).unwrap())
                    .font(self.icons_font())
                    .size(ICON_FONT_SIZE_TINY);
                if !selected {
                    icon.style(iced::theme::Text::Color(self.theme().palette().primary))
                } else {
                    icon
                }
            };
            iced::widget::button(
                iced::widget::row!(icon, text)
                    .align_items(iced::Alignment::Center)
                    .spacing(SPACING_NORMAL),
            )
            .on_press(MyMessage::Category(None))
            .style(RowButton::new(selected).into())
            .width(iced::Length::Fill)
            .into()
        };

        categories.insert(0, all);

        let column = iced::widget::column(categories)
            .height(iced::Length::Fill)
            .width(iced::Length::Fill);

        let container = iced::widget::container(column)
            .width(iced::Length::Fixed(SIDEBAR_WIDTH))
            .height(iced::Length::Shrink)
            // .padding([SPACING, SPACING, PADDING, SPACING])
            .center_x()
            .center_y();

        let scrollable = iced::widget::scrollable(container).direction(
            iced::widget::scrollable::Direction::Vertical(Default::default()),
        );

        let heading = iced::widget::container(
            iced::widget::text("Categories")
                .style(iced::theme::Text::Color(iced::Color {
                    a: 0.25,
                    ..self.theme().palette().text
                }))
                .size(FONT_SIZE_SMALLER)
                .font(self.bold_font()),
        )
        .padding([SPACING_NORMAL, 0, SPACING_NORMAL, 0]);

        iced::widget::container(iced::widget::column!(heading, scrollable))
            .style(CustomContainer::sidebar().move_to_style())
            .height(iced::Length::Fill)
            .padding([SPACING_NORMAL, SPACING_LARGE])
            .into()
    }

    fn view_toolbar_view_mode(&self) -> iced::Element<'_, MyMessage> {
        let list_view = {
            let icon = iced::widget::text(char::from_u32(CODEPOINT_LIST).unwrap())
                .font(self.icons_font())
                .size(ICON_FONT_SIZE_TOOLBAR);
            let button = iced::widget::button(icon).style(ToolbarButton::text().into());
            if self.grid_view {
                button.on_press(MyMessage::GridViewState(false))
            } else {
                button
            }
        };

        let grid_view = {
            let icon = iced::widget::text(char::from_u32(CODEPOINT_GRID).unwrap())
                .font(self.icons_font())
                .size(ICON_FONT_SIZE_TOOLBAR);
            let button = iced::widget::button(icon).style(ToolbarButton::text().into());
            if !self.grid_view {
                button.on_press(MyMessage::GridViewState(true))
            } else {
                button
            }
        };

        iced::widget::row!(grid_view, list_view)
            .align_items(iced::Alignment::Center)
            .into()
    }

    fn view_toolbar_search(&self) -> iced::Element<'_, MyMessage> {
        if !self.search_visible {
            let icon = iced::widget::text(char::from_u32(CODEPOINT_SEARCH).unwrap())
                .font(self.icons_font())
                .size(ICON_FONT_SIZE_TOOLBAR);
            let button = iced::widget::button(icon)
                .on_press(MyMessage::SearchVisibleState(true))
                .style(ToolbarButton::text().into());
            iced::widget::row!(button)
        } else {
            let text_input = iced::widget::text_input("Search", &self.search_text)
                .on_input(MyMessage::Search)
                .width(iced::Length::Fixed(200.0))
                .id(iced::widget::text_input::Id::new(SEARCH_TEXT_INPUT_ID));
            let text_input =
                text_input_wrapper::my_text_input_wrapper(text_input, MyMessage::SearchFocusState);
            let icon = iced::widget::text(char::from_u32(CODEPOINT_CLOSE).unwrap())
                .font(self.icons_font())
                .size(ICON_FONT_SIZE_TOOLBAR);
            let button = iced::widget::button(icon)
                .on_press(MyMessage::SearchVisibleState(false))
                .style(ToolbarButton::text().into());
            iced::widget::row!(text_input, button)
        }
        .align_items(iced::Alignment::Center)
        .into()
    }

    fn view_toolbar_active_category_and_count(&self) -> iced::Element<'_, MyMessage> {
        let active_categor = if self.search_text.is_empty() {
            iced::widget::text(
                self.selected_category
                    .as_ref()
                    .map(|v| capitalized_string(v))
                    .unwrap_or(String::from("All")),
            )
        } else {
            iced::widget::text("Search All")
        }
        .font(self.bold_font())
        .size(FONT_SIZE_SMALL);
        let visible_count = iced::widget::text(&format!("{} icons", self.visible_count()))
            .size(FONT_SIZE_SMALL)
            .font(self.font());
        iced::widget::column!(active_categor, visible_count).into()
    }

    fn view_toolbar(&self) -> iced::Element<'_, MyMessage> {
        let row = iced::widget::row!(
            self.view_toolbar_active_category_and_count(),
            iced::widget::container("").width(iced::Length::Fill),
            self.view_toolbar_view_mode(),
            self.view_toolbar_search(),
        )
        .spacing(SPACING_LARGE)
        .align_items(iced::Alignment::Center);

        iced::widget::container(row)
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .style(CustomContainer::toolbar().move_to_style())
            .center_x()
            .center_y()
            .padding([SPACING_NORMAL, SPACING_LARGE])
            .into()
    }

    fn view_content(&self) -> iced::Element<'_, MyMessage> {
        let icon_list_or_grid = if self.grid_view {
            self.view_icon_grid()
        } else {
            self.view_icon_list()
        };

        if let Some(codepoint) = self.codepoint {
            let item = match self.meta_list.get_item(codepoint) {
                Some(item) => item,
                None => panic!("This should not be possible"),
            };

            let preview = iced::widget::container(self.view_item_preview(item))
                .style(CustomContainer::preview().move_to_style())
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .center_x()
                .center_y();

            iced::widget::row!(
                icon_list_or_grid,
                iced::widget::vertical_rule(0).style(CustomRule::dark().move_to_style()),
                preview,
            )
        } else {
            iced::widget::row!(icon_list_or_grid)
        }
        .into()
    }

    fn get_items_per_row(&self) -> usize {
        let value: usize = match self.window_size.0 {
            1300u32..=u32::MAX => 8usize,
            1200u32..=1299 => 7usize,
            1100u32..=1199 => 6usize,
            1000u32..=1099 => 5usize,
            900u32..=999 => 4usize,
            _ => 3usize,
        };

        if self.codepoint.is_some() {
            let value = value / 2;
            if value > 1 {
                value
            } else {
                2
            }
        } else {
            value
        }
    }

    fn view_icon_grid(&self) -> iced::Element<'_, MyMessage> {
        let items = self
            .meta_list
            .items()
            .iter()
            .filter(|item| self.filter_item(item))
            .map(|item| self.view_item_preview_medium(item))
            .collect::<Vec<iced::Element<'_, MyMessage>>>();

        let items_per_row: usize = self.get_items_per_row();

        let mut column = iced::widget::column(vec![]);
        let mut row = iced::widget::row(vec![]);
        let mut last_index: usize = 0;
        for (index, item) in items.into_iter().enumerate() {
            last_index = index % items_per_row;
            if last_index != 0 {
                row = row.push(item);
            } else {
                last_index = 0;
                let tmp = row.spacing(SPACING_LARGE);
                column = column.push(
                    tmp.width(iced::Length::Fill)
                        .align_items(iced::Alignment::Center),
                );
                row = iced::widget::row(vec![item]);
            }
        }
        while last_index < items_per_row - 1 {
            last_index += 1;
            row = row.push(
                iced::widget::container("")
                    .width(iced::Length::Fill)
                    .padding(SPACING_LARGE),
            );
        }
        column = column.push(row.spacing(SPACING_LARGE));
        column = column.padding([0, SPACING_LARGE]).spacing(SPACING_LARGE);

        iced::widget::scrollable(
            iced::widget::container(column)
                .style(CustomContainer::default().move_to_style())
                .padding([0, 0, SPACING_LARGE, 0]),
        )
        .direction(iced::widget::scrollable::Direction::Vertical(
            Default::default(),
        ))
        .width(iced::Length::Fill)
        .id(iced::widget::scrollable::Id::new(SCROLLABLE_ICON_LIST_ID))
        .into()
    }

    fn filter_item(&self, item: &MaterialFontMeta) -> bool {
        if self.search_text.is_empty() {
            if let Some(category) = &self.selected_category {
                if !item.contains_category(category) {
                    return false;
                }
            }
            return true;
        }

        if self.search_text.is_empty() {
            return true;
        }

        if item.name().starts_with(&self.search_text)
            || item.contains_tag(&self.search_text)
            || item.matches_hex_codepoint(&self.search_text)
            || item.matches_codepoint(&self.search_text)
        {
            return true;
        }

        false
    }

    fn visible_count(&self) -> usize {
        self.meta_list
            .items()
            .iter()
            .filter(|item| self.filter_item(item))
            .count()
    }

    fn view_icon_list(&self) -> iced::Element<'_, MyMessage> {
        let items = self
            .meta_list
            .items()
            .iter()
            .filter(|item| self.filter_item(item))
            .map(|item| self.view_item_preview_small(item))
            .collect::<Vec<iced::Element<'_, MyMessage>>>();

        let column = iced::widget::column(items)
            .width(iced::Length::Fill)
            .spacing(SPACING_SMALL)
            .padding([SPACING_NORMAL, SPACING_LARGE]);

        iced::widget::scrollable(
            iced::widget::container(column).style(CustomContainer::default().move_to_style()),
        )
        .direction(iced::widget::scrollable::Direction::Vertical(
            Default::default(),
        ))
        .width(iced::Length::Fill)
        .id(iced::widget::scrollable::Id::new(SCROLLABLE_ICON_LIST_ID))
        .into()
    }

    fn view_item_preview(&self, item: &MaterialFontMeta) -> iced::Element<'_, MyMessage> {
        let previewed_icon = iced::widget::text(item.to_char())
            .font(self.icons_font())
            .size(ICON_FONT_SIZE_BIG);
        let name = {
            let label = iced::widget::text("Name:")
                .font(self.font())
                .size(FONT_SIZE_LARGE);
            let value = iced::widget::text(item.name())
                .font(self.font())
                .size(FONT_SIZE_LARGE);
            let (codepoint, icon_style) = self
                .copy_animation_info
                .map(|info| {
                    if let CopyType::Name = info.copy_type() {
                        (
                            CODEPOINT_SUCCESS,
                            iced::theme::Text::Color(self.theme().palette().success),
                        )
                    } else {
                        (CODEPOINT_COPY, iced::theme::Text::Default)
                    }
                })
                .unwrap_or((CODEPOINT_COPY, iced::theme::Text::Default));
            let icon = iced::widget::text(char::from_u32(codepoint).unwrap())
                .style(icon_style)
                .font(self.icons_font())
                .size(ICON_FONT_SIZE_SMALLER);
            let button = iced::widget::button(icon)
                .on_press(MyMessage::Copy(
                    item.name().clone(),
                    AnimationInfo::name(COPY_ANIMATION_STEPS),
                ))
                .style(ToolbarButton::text().into())
                .padding(0);
            iced::widget::row!(button, label, value,)
                .align_items(iced::Alignment::Center)
                .width(iced::Length::Fill)
                .height(iced::Length::Shrink)
                .spacing(SPACING_NORMAL)
        };
        let codepoint_hex = {
            let label = iced::widget::text("Codepoint (hex):")
                .font(self.font())
                .size(FONT_SIZE_LARGE);
            let value = iced::widget::text(item.to_hex_codepoint())
                .font(self.font())
                .size(FONT_SIZE_LARGE);
            let (codepoint, icon_style) = self
                .copy_animation_info
                .map(|info| {
                    if let CopyType::HexCodepoint = info.copy_type() {
                        (
                            CODEPOINT_SUCCESS,
                            iced::theme::Text::Color(self.theme().palette().success),
                        )
                    } else {
                        (CODEPOINT_COPY, iced::theme::Text::Default)
                    }
                })
                .unwrap_or((CODEPOINT_COPY, iced::theme::Text::Default));
            let icon = iced::widget::text(char::from_u32(codepoint).unwrap())
                .font(self.icons_font())
                .style(icon_style)
                .size(ICON_FONT_SIZE_SMALLER);
            let button = iced::widget::button(icon)
                .on_press(MyMessage::Copy(
                    item.to_hex_codepoint(),
                    AnimationInfo::hex_codepoint(COPY_ANIMATION_STEPS),
                ))
                .style(ToolbarButton::text().into())
                .padding(0);
            iced::widget::row!(button, label, value,)
                .align_items(iced::Alignment::Center)
                .width(iced::Length::Fill)
                .height(iced::Length::Shrink)
                .spacing(SPACING_NORMAL)
        };
        let codepoint = {
            let label = iced::widget::text("Codepoint (u32):")
                .font(self.font())
                .size(FONT_SIZE_LARGE);
            let value = iced::widget::text(item.codepoint().to_string())
                .font(self.font())
                .size(FONT_SIZE_LARGE);
            let (codepoint, icon_style) = self
                .copy_animation_info
                .map(|info| {
                    if let CopyType::Codepoint = info.copy_type() {
                        (
                            CODEPOINT_SUCCESS,
                            iced::theme::Text::Color(self.theme().palette().success),
                        )
                    } else {
                        (CODEPOINT_COPY, iced::theme::Text::Default)
                    }
                })
                .unwrap_or((CODEPOINT_COPY, iced::theme::Text::Default));
            let icon = iced::widget::text(char::from_u32(codepoint).unwrap())
                .style(icon_style)
                .font(self.icons_font())
                .size(ICON_FONT_SIZE_SMALLER);
            let button = iced::widget::button(icon)
                .on_press(MyMessage::Copy(
                    item.codepoint().to_string(),
                    AnimationInfo::codepoint(COPY_ANIMATION_STEPS),
                ))
                .style(ToolbarButton::text().into())
                .padding(0);
            iced::widget::row!(button, label, value,)
                .align_items(iced::Alignment::Center)
                .width(iced::Length::Fill)
                .height(iced::Length::Shrink)
                .spacing(SPACING_NORMAL)
        };
        let close_button = {
            let icon = iced::widget::text(char::from_u32(CODEPOINT_CLOSE_CIRCLE).unwrap())
                .font(self.icons_font())
                .size(ICON_FONT_SIZE_SMALL);
            iced::widget::button(icon)
                .on_press(MyMessage::Codepoint(None))
                .style(ToolbarButton::text().into())
        };
        iced::widget::column!(previewed_icon, name, codepoint_hex, codepoint, close_button)
            .spacing(SPACING_NORMAL)
            .align_items(iced::Alignment::Center)
            .padding([SPACING_LARGE, SPACING_EXTRA_LARGE])
            .into()
    }

    fn view_item_preview_small(&self, item: &MaterialFontMeta) -> iced::Element<'_, MyMessage> {
        let selected = if let Some(current) = self.codepoint {
            item.codepoint() == current
        } else {
            false
        };
        let icon = iced::widget::text(item.to_char())
            .font(self.icons_font())
            .size(ICON_FONT_SIZE_SMALL);
        let name = iced::widget::text(item.name())
            .font(self.selected_font(selected))
            .size(FONT_SIZE_STANDARD);
        let row = iced::widget::row!(icon, name)
            .spacing(SPACING_NORMAL)
            .align_items(iced::Alignment::Center);
        iced::widget::button(row)
            .on_press(MyMessage::Codepoint(Some(item.codepoint())))
            .style(RowButton::new(selected).into())
            .width(iced::Length::Fill)
            .padding(0)
            .into()
    }

    fn view_item_preview_medium(&self, item: &MaterialFontMeta) -> iced::Element<'_, MyMessage> {
        let selected = if let Some(current) = self.codepoint {
            item.codepoint() == current
        } else {
            false
        };
        let icon = iced::widget::text(item.to_char())
            .font(self.icons_font())
            .size(ICON_FONT_SIZE_MEDIUM);
        let name = iced::widget::text(item.name())
            .font(self.selected_font(selected))
            .size(FONT_SIZE_STANDARD);
        let column = iced::widget::column!(icon, name).align_items(iced::Alignment::Center);
        iced::widget::button(column)
            .on_press(MyMessage::Codepoint(Some(item.codepoint())))
            .style(RowButton::new_bordered(selected).into())
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .padding(SPACING_LARGE)
            .into()
    }
}

impl iced::Application for MyApp {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = MyMessage;
    type Theme = iced::theme::Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let mut commands: Vec<iced::Command<MyMessage>> =
            vec![ICONS_FONT_BYTES, FONT_BYTES_REGULAR, FONT_BYTES_BOLD]
                .iter()
                .map(|&bytes| {
                    iced::font::load(std::borrow::Cow::from(bytes)).map(|r| {
                        if let Err(e) = r {
                            panic!("{:?}", e);
                        }
                        MyMessage::FontLoaded
                    })
                })
                .collect();

        commands.push(iced::Command::perform(
            MaterialFontMetaList::load_from_bytes_fake_async(std::borrow::Cow::from(
                ICONS_META_BYTES,
            )),
            |r: Result<MaterialFontMetaList, LoadError>| match r {
                Err(e) => panic!("{:?}", e),
                Ok(list) => MyMessage::MetaListLoaded(list),
            },
        ));

        (Default::default(), iced::Command::batch(commands))
    }

    fn title(&self) -> String {
        APP_TITLE.into()
    }

    fn theme(&self) -> Self::Theme {
        self.custom_theme.to_theme()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let mut subs: Vec<iced::Subscription<Self::Message>> =
            vec![iced::subscription::events().map(|e| MyMessage::Event(e))];

        if self.copy_animation_info.is_some() {
            let sub = iced::time::every(iced::time::Duration::from_millis(COPY_ANIMATION_TICKS_MS))
                .map(|_| MyMessage::CopiedAnimationTick);
            subs.push(sub);
        }

        iced::subscription::Subscription::batch(subs)
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        if !self.are_resources_loaded() {
            return iced::widget::container("")
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .center_x()
                .center_y()
                .padding(SPACING_LARGE)
                .into();
        }

        iced::widget::row!(
            self.view_sidebar(),
            iced::widget::vertical_rule(0).style(CustomRule::dark().move_to_style()),
            iced::widget::column!(
                self.view_toolbar(),
                iced::widget::horizontal_rule(0).style(CustomRule::dark().move_to_style()),
                self.view_content(),
            )
        )
        .into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            MyMessage::CopiedAnimationTick => {
                if let Some(info) = self.copy_animation_info.take() {
                    let updated = info.advance();
                    if !updated.is_done() {
                        self.copy_animation_info = Some(updated);
                    }
                }
                iced::Command::none()
            }
            MyMessage::Event(e) => {
                if let iced::event::Event::Window(iced::window::Event::Resized { width, height }) =
                    e
                {
                    self.window_size = (width, height);
                }
                if let iced::event::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key_code: iced::keyboard::KeyCode::Escape,
                    ..
                }) = e
                {
                    if self.codepoint.is_some() {
                        self.codepoint = None;
                    } else if self.search_visible {
                        self.search_visible = false;
                        self.search_text = "".into();
                    }
                }
                if let iced::event::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key_code: iced::keyboard::KeyCode::F,
                    modifiers: iced::keyboard::Modifiers::CTRL,
                }) = e
                {
                    if self.search_visible {
                        return iced::widget::text_input::focus(iced::widget::text_input::Id::new(
                            SEARCH_TEXT_INPUT_ID,
                        ));
                    } else {
                        return iced::Command::perform(
                            type_to_async(MyMessage::SearchVisibleState(true)),
                            |m| m,
                        );
                    }
                }
                iced::Command::none()
            }
            MyMessage::GridViewState(grid_view) => {
                self.grid_view = grid_view;
                self.codepoint = None;
                self.search_visible = false;
                self.search_text = "".into();
                iced::widget::scrollable::snap_to(
                    iced::widget::scrollable::Id::new(SCROLLABLE_ICON_LIST_ID),
                    iced::widget::scrollable::RelativeOffset::START,
                )
            }
            MyMessage::Copy(s, animation_info) => {
                self.copy_animation_info = Some(animation_info);
                iced::clipboard::write(s)
            }
            MyMessage::SearchFocusState(is_focused) => {
                if !is_focused && self.search_visible && self.search_text.is_empty() {
                    // println!("Forcing search to hide");
                    self.search_visible = false;
                    self.search_text = "".into();
                    // iced::widget::scrollable::snap_to(
                    //     iced::widget::scrollable::Id::new(SCROLLABLE_ICON_LIST_ID),
                    //     iced::widget::scrollable::RelativeOffset::START,
                    // )
                }
                iced::Command::none()
            }
            MyMessage::SearchVisibleState(visible) => {
                self.search_visible = visible;
                self.codepoint = None;
                if !visible {
                    self.search_text = "".into();
                    iced::Command::none()
                } else {
                    iced::widget::text_input::focus(iced::widget::text_input::Id::new(
                        SEARCH_TEXT_INPUT_ID,
                    ))
                }
            }
            MyMessage::Codepoint(codepoint) => {
                self.codepoint = codepoint;
                iced::Command::none()
            }
            MyMessage::Search(text) => {
                self.search_text = text;
                self.codepoint = None;
                iced::widget::scrollable::snap_to(
                    iced::widget::scrollable::Id::new(SCROLLABLE_ICON_LIST_ID),
                    iced::widget::scrollable::RelativeOffset::START,
                )
            }
            MyMessage::Category(category) => {
                self.selected_category = category;
                self.codepoint = None;
                self.search_visible = false;
                self.search_text = "".into();
                iced::widget::scrollable::snap_to(
                    iced::widget::scrollable::Id::new(SCROLLABLE_ICON_LIST_ID),
                    iced::widget::scrollable::RelativeOffset::START,
                )
            }
            MyMessage::FontLoaded => {
                self.loaded_resources_count += 1;
                iced::Command::none()
            }
            MyMessage::MetaListLoaded(meta_list) => {
                self.meta_list = meta_list;
                self.loaded_resources_count += 1;
                iced::Command::none()
            }
        }
    }
}

mod copy_to_clipboard_animation {
    #[derive(Clone, Debug, Copy)]
    pub struct Progress(f32);

    #[derive(Copy, Debug, Clone, PartialEq)]
    pub enum CopyType {
        Name,
        Codepoint,
        HexCodepoint,
    }

    #[derive(Copy, Debug, Clone)]
    pub struct AnimationInfo {
        progress: Progress,
        copy_type: CopyType,
        steps: u64,
    }

    impl AnimationInfo {
        pub fn copy_type(&self) -> CopyType {
            self.copy_type
        }

        pub fn is_done(&self) -> bool {
            self.progress.0 == 1.0
        }

        fn new(copy_type: CopyType, steps: u64) -> Self {
            if steps == 0 {
                panic!("Steps should be more than 0");
            }
            Self {
                progress: Progress(0.0),
                copy_type,
                steps,
            }
        }

        pub fn name(steps: u64) -> Self {
            Self::new(CopyType::Name, steps)
        }

        pub fn codepoint(steps: u64) -> Self {
            Self::new(CopyType::Codepoint, steps)
        }

        pub fn hex_codepoint(steps: u64) -> Self {
            Self::new(CopyType::HexCodepoint, steps)
        }

        pub fn advance(self) -> Self {
            let progress = (self.progress.0 + (1.0f32 / (self.steps as f32))).min(1.0);
            Self {
                progress: Progress(progress),
                ..self
            }
        }
    }
}
