use iced::Application as _;

use models::{LoadError, MaterialFontMeta, MaterialFontMetaList};

mod models;

const APP_TITLE: &'static str = "Iced Material Icons Browser";
const ICONS_FONT_BYTES: &[u8] = include_bytes!("../resources/MaterialIcons-Regular.ttf");
const ICONS_FONT_NAME: &'static str = "Material Icons";
const ICONS_FONT_META_FILE_PATH: &'static str = "./resources/2023-09-12-material-icons-meta.json";

const SPACING: u16 = 5;
const PADDING: u16 = 20;
const ICON_FONT_SIZE_BIG: u16 = 100;
const ICON_FONT_SIZE_SMALL: u16 = 24;
const SIDEBAR_WIDTH: f32 = 200.0;

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
    icon_index: usize,
    selected_category: Option<String>,
    search_text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            icons_font: Default::default(),
            meta_list: Default::default(),
            loaded_resources_count: 0,
            icon_index: 1110,
            selected_category: None,
            search_text: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
enum MyMessage {
    IconsFontLoaded(iced::Font),
    MetaListLoaded(MaterialFontMetaList),
    IndexDown,
    IndexUp,
    Category(Option<String>),
    Search(String),
}

impl MyApp {
    fn are_resources_loaded(&self) -> bool {
        self.loaded_resources_count == 2
    }

    fn view_sidebar(&self) -> iced::Element<'_, MyMessage> {
        let categories = self
            .meta_list
            .categories()
            .iter()
            .map(|name| {
                let text = iced::widget::text(name);
                iced::widget::button(text)
                    .on_press(MyMessage::Category(Some(name.clone())))
                    .style(iced::theme::Button::Text)
                    .into()
            })
            .collect::<Vec<iced::Element<'_, MyMessage>>>();

        let column = iced::widget::column(categories)
            .height(iced::Length::Fill)
            .width(iced::Length::Fill);

        let container = iced::widget::container(column)
            .width(iced::Length::Fixed(SIDEBAR_WIDTH))
            .height(iced::Length::Shrink)
            .padding(PADDING)
            .center_x()
            .center_y();

        iced::widget::scrollable(container)
            .direction(iced::widget::scrollable::Direction::Vertical(
                Default::default(),
            ))
            .into()
    }

    fn view_toolbar(&self) -> iced::Element<'_, MyMessage> {
        let search = iced::widget::text_input("[loading]", &self.search_text).on_input(MyMessage::Search);

        let row = iced::widget::row!(
            iced::widget::container("").width(iced::Length::Fill),
            self.view_controls(),
            search,
        )
        .spacing(SPACING)
        .align_items(iced::Alignment::Center);

        iced::widget::container(row)
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .center_x()
            .center_y()
            .padding(PADDING)
            .into()
    }

    fn view_content(&self) -> iced::Element<'_, MyMessage> {
        let item = match self.meta_list.items().get(self.icon_index) {
            Some(item) => item,
            None => panic!("Please handle this"),
        };

        let preview = iced::widget::container(self.view_item_preview(item))
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y();

        iced::widget::row!(self.view_icon_list(), preview).into()
    }

    fn view_icon_list(&self) -> iced::Element<'_, MyMessage> {
        let items = self
            .meta_list
            .items()
            .iter()
            .filter(|item| {
                let Some(category) = &self.selected_category else {
                    return true;
                };
                item.contains_category(category)
            })
            .map(|item| self.view_item_preview_small(item))
            .collect::<Vec<iced::Element<'_, MyMessage>>>();

        let column = iced::widget::column(items)
            .width(iced::Length::Fill)
            .spacing(SPACING)
            .padding([SPACING, PADDING]);

        iced::widget::scrollable(column)
            .direction(iced::widget::scrollable::Direction::Vertical(
                Default::default(),
            ))
            .width(iced::Length::Fill)
            .into()
    }

    fn view_controls(&self) -> iced::Element<'_, MyMessage> {
        let down = {
            let button = iced::widget::button("-");
            if self.icon_index > 0 {
                button.on_press(MyMessage::IndexDown)
            } else {
                button
            }
        };

        let up = {
            let button = iced::widget::button("+");
            if self.meta_list.count() - 1 > self.icon_index {
                button.on_press(MyMessage::IndexUp)
            } else {
                button
            }
        };

        let index = iced::widget::text(format!("Index = {}", self.icon_index));

        iced::widget::row!(down, index, up)
            .spacing(SPACING)
            .align_items(iced::Alignment::Center)
            .into()
    }

    fn view_item_preview(&self, item: &MaterialFontMeta) -> iced::Element<'_, MyMessage> {
        let icon = iced::widget::text(item.to_char())
            .font(self.icons_font)
            .size(ICON_FONT_SIZE_BIG);
        let name = iced::widget::text(format!("{} ({})", item.name(), item.to_hex_codepoint()));
        iced::widget::column!(icon, name)
            .spacing(SPACING)
            .align_items(iced::Alignment::Center)
            .into()
    }

    fn view_item_preview_small(&self, item: &MaterialFontMeta) -> iced::Element<'_, MyMessage> {
        let icon = iced::widget::text(item.to_char())
            .font(self.icons_font)
            .size(ICON_FONT_SIZE_SMALL);
        let name = iced::widget::text(format!("{} ({})", item.name(), item.to_hex_codepoint()));
        iced::widget::row!(icon, name)
            .spacing(SPACING)
            .align_items(iced::Alignment::Center)
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
                    MaterialFontMetaList::load_async(ICONS_FONT_META_FILE_PATH),
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
        iced::theme::Theme::Dark
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        if !self.are_resources_loaded() {
            return iced::widget::container("")
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .center_x()
                .center_y()
                .padding(PADDING)
                .into();
        }

        iced::widget::row!(
            self.view_sidebar(),
            // iced::widget::vertical_rule(SPACING),
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
            MyMessage::Search(text) => {
                self.search_text = text;
                iced::Command::none()
            }
            MyMessage::Category(category) => {
                self.selected_category = category;
                iced::Command::none()
            }
            MyMessage::IndexDown => {
                self.icon_index -= 1;
                iced::Command::none()
            }
            MyMessage::IndexUp => {
                self.icon_index += 1;
                iced::Command::none()
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
