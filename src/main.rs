use iced::Application as _;

use models::{LoadError, MaterialFontMeta, MaterialFontMetaList};
use styling::{CustomTheme, RowButton, ToolbarButton};

mod models;
mod styling;

const APP_TITLE: &'static str = "Iced Material Icons Browser";
const ICONS_FONT_NAME: &'static str = "Material Icons";

const ICONS_FONT_BYTES: &[u8] = include_bytes!("../resources/MaterialIcons-Regular.ttf");
const ICONS_META_BYTES: &[u8] = include_bytes!("../resources/2023-09-12-material-icons-meta.json");

const SCROLLABLE_ICON_LIST_ID: &'static str = "scrollable_icon_list_id";
const SEARCH_TEXT_INPUT_ID: &'static str = "search_text_input_id";

const SMALL_SPACING: u16 = 5;
const SPACING: u16 = 10;
const PADDING: u16 = 20;
const ICON_FONT_SIZE_BIG: u16 = 96;
const ICON_FONT_SIZE_SMALL: u16 = 24;
const ICON_FONT_SIZE_SMALLER: u16 = 20;
const ICON_FONT_SIZE_TOOLBAR: u16 = 24;
const SIDEBAR_WIDTH: f32 = 200.0;

async fn type_to_async<T>(t: T) -> T {
    t
}

fn main() -> iced::Result {
    MyApp::run(iced::Settings {
        window: iced::window::Settings {
            position: iced::window::Position::Specific(20, 900),
            size: (1000, 500),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Clone, Debug)]
struct MyApp {
    icons_font: iced::Font,
    meta_list: MaterialFontMetaList,
    loaded_resources_count: usize,
    selected_category: Option<String>,
    search_text: String,
    search_visible: bool,
    codepoint: Option<u32>,
    font: iced::Font,
    custom_theme: CustomTheme,
    grid_view: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            icons_font: Default::default(),
            meta_list: Default::default(),
            loaded_resources_count: 0,
            selected_category: None,
            search_text: String::new(),
            search_visible: false,
            codepoint: None,
            font: iced::Font::with_name("Arial"),
            custom_theme: Default::default(),
            grid_view: false,
        }
    }
}

#[derive(Clone, Debug)]
enum MyMessage {
    IconsFontLoaded(iced::Font),
    MetaListLoaded(MaterialFontMetaList),
    Category(Option<String>),
    Search(String),
    SearchVisibleState(bool),
    GridViewState(bool),
    CodePoint(Option<u32>),
    IconListShouldScrollTop,
    ThemeMode,
    Copy(String),
}

impl MyApp {
    fn bold_font(&self) -> iced::Font {
        iced::Font {
            weight: iced::font::Weight::Bold,
            ..self.font
        }
    }

    fn selected_font(&self, selected: bool) -> iced::Font {
        if selected {
            self.bold_font()
        } else {
            self.font
        }
    }

    fn are_resources_loaded(&self) -> bool {
        self.loaded_resources_count == 2
    }

    fn view_sidebar(&self) -> iced::Element<'_, MyMessage> {
        let mut categories = self
            .meta_list
            .categories()
            .iter()
            .map(|name| {
                let selected = if let Some(current) = self.selected_category.as_ref() {
                    name == current
                } else {
                    false
                };
                let text = iced::widget::text(name).font(self.selected_font(selected));
                iced::widget::button(text)
                    .on_press(MyMessage::Category(Some(name.clone())))
                    .style(RowButton::new(selected).into())
                    .width(iced::Length::Fill)
                    .into()
            })
            .collect::<Vec<iced::Element<'_, MyMessage>>>();

        let all = {
            let selected = self.selected_category.is_none();
            let text = iced::widget::text("all").font(self.selected_font(selected));
            iced::widget::button(text)
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
            .padding([SPACING, SPACING, PADDING, SPACING])
            .center_x()
            .center_y();

        let scrollable = iced::widget::scrollable(container).direction(
            iced::widget::scrollable::Direction::Vertical(Default::default()),
        );

        let heading = iced::widget::container(
            iced::widget::text("Categories".to_uppercase())
                .size(11.0)
                .font(self.bold_font()),
        )
        .padding([SPACING, SPACING, 0, SPACING]);

        iced::widget::column!(heading, scrollable).into()
    }

    fn view_toolbar_theme_toggler(&self) -> iced::Element<'_, MyMessage> {
        let codepoint: u32 = if let CustomTheme::Dark = self.custom_theme {
            58648
        } else {
            58652
        };
        let icon = iced::widget::text(char::from_u32(codepoint).unwrap())
            .font(self.icons_font)
            .size(ICON_FONT_SIZE_TOOLBAR);

        iced::widget::button(icon)
            .style(ToolbarButton::default().into())
            .on_press(MyMessage::ThemeMode)
            .into()
    }

    fn view_toolbar_view_mode(&self) -> iced::Element<'_, MyMessage> {
        let list_view = {
            let icon = iced::widget::text(char::from_u32(57921).unwrap())
                .font(self.icons_font)
                .size(ICON_FONT_SIZE_TOOLBAR);
            let button = iced::widget::button(icon).style(ToolbarButton::default().into());
            if self.grid_view {
                button.on_press(MyMessage::GridViewState(false))
            } else {
                button
            }
        };

        let grid_view = {
            let icon = iced::widget::text(char::from_u32(59824).unwrap())
                .font(self.icons_font)
                .size(ICON_FONT_SIZE_TOOLBAR);
            let button = iced::widget::button(icon).style(ToolbarButton::default().into());
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
            let icon = iced::widget::text(char::from_u32(59574).unwrap())
                .font(self.icons_font)
                .size(ICON_FONT_SIZE_TOOLBAR);
            let button = iced::widget::button(icon)
                .on_press(MyMessage::SearchVisibleState(true))
                .style(ToolbarButton::default().into());
            iced::widget::row!(button)
        } else {
            let text_input = iced::widget::text_input("Search", &self.search_text)
                .on_input(MyMessage::Search)
                .width(iced::Length::Fixed(200.0))
                .id(iced::widget::text_input::Id::new(SEARCH_TEXT_INPUT_ID));
            let icon = iced::widget::text(char::from_u32(58829).unwrap())
                .font(self.icons_font)
                .size(ICON_FONT_SIZE_TOOLBAR);
            let button = iced::widget::button(icon)
                .on_press(MyMessage::SearchVisibleState(false))
                .style(ToolbarButton::default().into());
            iced::widget::row!(button, text_input)
        }
        .align_items(iced::Alignment::Center)
        .into()
    }

    fn view_toolbar_active_category_and_count(&self) -> iced::Element<'_, MyMessage> {
        let active_categor = iced::widget::text(
            self.selected_category
                .as_ref()
                .unwrap_or(&String::from("All")),
        )
        .font(self.bold_font())
        .size(14.0);
        let visible_count = iced::widget::text(&format!("{} icons", self.meta_list.count()))
            .size(14.0)
            .font(self.font);
        iced::widget::column!(active_categor, visible_count).into()
    }

    fn view_toolbar(&self) -> iced::Element<'_, MyMessage> {
        let row = iced::widget::row!(
            self.view_toolbar_active_category_and_count(),
            iced::widget::container("").width(iced::Length::Fill),
            self.view_toolbar_theme_toggler(),
            self.view_toolbar_view_mode(),
            self.view_toolbar_search(),
        )
        .spacing(SPACING)
        .align_items(iced::Alignment::Center);

        iced::widget::container(row)
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .center_x()
            .center_y()
            .padding([SPACING, PADDING])
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
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .center_x()
                .center_y();

            iced::widget::row!(
                icon_list_or_grid,
                iced::widget::vertical_rule(SPACING),
                preview,
            )
        } else {
            iced::widget::row!(icon_list_or_grid)
        }
        .into()
    }

    fn view_icon_grid(&self) -> iced::Element<'_, MyMessage> {
        iced::widget::container("[grid view]")
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn view_icon_list(&self) -> iced::Element<'_, MyMessage> {
        let items = self
            .meta_list
            .items()
            .iter()
            .filter(|item| {
                if let Some(category) = &self.selected_category {
                    if !item.contains_category(category) {
                        return false;
                    }
                }

                if self.search_text.is_empty() {
                    return true;
                }

                if item.name().starts_with(&self.search_text)
                    || item.contains_tag(&self.search_text)
                    || item.matches_hex_codepoint(&self.search_text)
                {
                    return true;
                }

                false
            })
            .map(|item| self.view_item_preview_small(item))
            .collect::<Vec<iced::Element<'_, MyMessage>>>();

        let column = iced::widget::column(items)
            .width(iced::Length::Fill)
            .spacing(SMALL_SPACING)
            .padding([SPACING, PADDING]);

        iced::widget::scrollable(column)
            .direction(iced::widget::scrollable::Direction::Vertical(
                Default::default(),
            ))
            .width(iced::Length::Fill)
            .id(iced::widget::scrollable::Id::new(SCROLLABLE_ICON_LIST_ID))
            .into()
    }

    fn view_item_preview(&self, item: &MaterialFontMeta) -> iced::Element<'_, MyMessage> {
        let previewed_icon = iced::widget::text(item.to_char())
            .font(self.icons_font)
            .size(ICON_FONT_SIZE_BIG);
        let name = {
            let label = iced::widget::text("Name:").font(self.font);
            let value = iced::widget::text(item.name()).font(self.font);
            let icon = iced::widget::text(char::from_u32(57677).unwrap())
                .font(self.icons_font)
                .size(ICON_FONT_SIZE_SMALLER);
            let button = iced::widget::button(icon)
                .on_press(MyMessage::Copy(item.name().clone()))
                .style(ToolbarButton::text().into())
                .padding(0);
            iced::widget::row!(button, label, value,)
                .align_items(iced::Alignment::Center)
                .width(iced::Length::Fill)
                .height(iced::Length::Shrink)
                .spacing(SPACING)
        };
        let codepoint_hex = {
            let label = iced::widget::text("Codepoint (hex):").font(self.font);
            let value = iced::widget::text(item.to_hex_codepoint()).font(self.font);
            let icon = iced::widget::text(char::from_u32(57677).unwrap())
                .font(self.icons_font)
                .size(ICON_FONT_SIZE_SMALLER);
            let button = iced::widget::button(icon)
                .on_press(MyMessage::Copy(item.to_hex_codepoint()))
                .style(ToolbarButton::text().into())
                .padding(0);
            iced::widget::row!(button, label, value,)
                .align_items(iced::Alignment::Center)
                .width(iced::Length::Fill)
                .height(iced::Length::Shrink)
                .spacing(SPACING)
        };
        let codepoint = {
            let label = iced::widget::text("Codepoint (u32):").font(self.font);
            let value = iced::widget::text(item.codepoint().to_string()).font(self.font);
            let icon = iced::widget::text(char::from_u32(57677).unwrap())
                .font(self.icons_font)
                .size(ICON_FONT_SIZE_SMALLER);
            let button = iced::widget::button(icon)
                .on_press(MyMessage::Copy(item.codepoint().to_string()))
                .style(ToolbarButton::text().into())
                .padding(0);
            iced::widget::row!(button, label, value,)
                .align_items(iced::Alignment::Center)
                .width(iced::Length::Fill)
                .height(iced::Length::Shrink)
                .spacing(SPACING)
        };
        iced::widget::column!(previewed_icon, name, codepoint_hex, codepoint)
            .spacing(SPACING)
            .align_items(iced::Alignment::Center)
            .padding(PADDING)
            .into()
    }

    fn view_item_preview_small(&self, item: &MaterialFontMeta) -> iced::Element<'_, MyMessage> {
        let selected = if let Some(current) = self.codepoint {
            item.codepoint() == current
        } else {
            false
        };
        let icon = iced::widget::text(item.to_char())
            .font(self.icons_font)
            .size(ICON_FONT_SIZE_SMALL);
        let name = iced::widget::text(item.name()).font(self.selected_font(selected));
        let row = iced::widget::row!(icon, name)
            .spacing(SPACING)
            .align_items(iced::Alignment::Center);
        iced::widget::button(row)
            .on_press(MyMessage::CodePoint(Some(item.codepoint())))
            .style(RowButton::new(selected).into())
            .width(iced::Length::Fill)
            .padding(0)
            .into()
    }
}

impl iced::Application for MyApp {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = MyMessage;
    type Theme = iced::theme::Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Default::default(),
            iced::Command::batch(vec![
                iced::font::load(std::borrow::Cow::from(ICONS_FONT_BYTES)).map(|r| {
                    if let Err(e) = r {
                        panic!("{:?}", e);
                    }
                    let font = iced::Font {
                        weight: iced::font::Weight::Normal,
                        family: iced::font::Family::Name(ICONS_FONT_NAME),
                        monospaced: false,
                        stretch: iced::font::Stretch::Normal,
                    };
                    MyMessage::IconsFontLoaded(font)
                }),
                iced::Command::perform(
                    MaterialFontMetaList::load_from_bytes_fake_async(std::borrow::Cow::from(
                        ICONS_META_BYTES,
                    )),
                    |r: Result<MaterialFontMetaList, LoadError>| match r {
                        Err(e) => panic!("{:?}", e),
                        Ok(list) => MyMessage::MetaListLoaded(list),
                    },
                ),
            ]),
        )
    }

    fn title(&self) -> String {
        APP_TITLE.into()
    }

    fn theme(&self) -> Self::Theme {
        self.custom_theme.to_theme()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        if !self.are_resources_loaded() {
            return iced::widget::container("Loading...")
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .center_x()
                .center_y()
                .padding(PADDING)
                .into();
        }

        iced::widget::row!(
            self.view_sidebar(),
            iced::widget::vertical_rule(SPACING),
            iced::widget::column!(
                self.view_toolbar(),
                iced::widget::horizontal_rule(SPACING),
                self.view_content(),
            )
        )
        .into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            MyMessage::GridViewState(grid_view) => {
                self.grid_view = grid_view;
                self.codepoint = None;
                iced::Command::batch(vec![
                    iced::Command::perform(
                        type_to_async(MyMessage::SearchVisibleState(false)),
                        |m| m,
                    ),
                    iced::Command::perform(
                        type_to_async(MyMessage::IconListShouldScrollTop),
                        |m| m,
                    ),
                ])
            }
            MyMessage::Copy(s) => iced::clipboard::write(s),
            MyMessage::ThemeMode => {
                self.custom_theme = self.custom_theme.toggle();
                iced::Command::none()
            }
            MyMessage::SearchVisibleState(visible) => {
                self.search_visible = visible;
                if !visible {
                    self.search_text = "".into();
                    iced::Command::none()
                } else {
                    iced::widget::text_input::focus(iced::widget::text_input::Id::new(
                        SEARCH_TEXT_INPUT_ID,
                    ))
                }
            }
            MyMessage::IconListShouldScrollTop => iced::widget::scrollable::snap_to(
                iced::widget::scrollable::Id::new(SCROLLABLE_ICON_LIST_ID),
                iced::widget::scrollable::RelativeOffset::START,
            ),
            MyMessage::CodePoint(codepoint) => {
                self.codepoint = codepoint;
                iced::Command::none()
            }
            MyMessage::Search(text) => {
                self.search_text = text;
                self.codepoint = None;
                iced::Command::none()
            }
            MyMessage::Category(category) => {
                self.selected_category = category;
                self.codepoint = None;
                iced::Command::batch(vec![
                    iced::Command::perform(
                        type_to_async(MyMessage::SearchVisibleState(false)),
                        |m| m,
                    ),
                    iced::Command::perform(
                        type_to_async(MyMessage::IconListShouldScrollTop),
                        |m| m,
                    ),
                ])
            }
            MyMessage::IconsFontLoaded(icons_font) => {
                self.icons_font = icons_font;
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
