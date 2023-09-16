use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct MaterialFontMeta {
    name: String,
    codepoint: u32,
    categories: Vec<String>,
    tags: Vec<String>,
}

impl MaterialFontMeta {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn codepoint(&self) -> u32 {
        self.codepoint
    }

    pub fn to_char(&self) -> char {
        char::from_u32(self.codepoint).unwrap()
    }

    pub fn to_hex_codepoint(&self) -> String {
        format!("{:04x}", self.codepoint)
    }

    pub fn contains_category(&self, category: &String) -> bool {
        self.categories.contains(category)
    }

    pub fn contains_tag(&self, tag: &String) -> bool {
        self.tags.contains(tag)
    }

    // pub fn matches_name(&self, name: &String) -> bool {
    //     &self.name == name
    // }

    // pub fn matches_any(&self, value: &String) -> bool {
    //     if self.matches_name(value) {
    //         return true;
    //     }
    //     if self.contains_category(value) {
    //         return true;
    //     }
    //     if self.contains_tag(value) {
    //         return true;
    //     }
    //     if self.matches_hex_codepoint(value) {
    //         return true;
    //     }
    //     false
    // }

    pub fn matches_hex_codepoint(&self, codepoint: &String) -> bool {
        let hex_codepoint = format!("{:08x}", self.codepoint);
        hex_codepoint.ends_with(codepoint)
    }
}

#[derive(Clone, Debug)]
pub struct MaterialFontMetaList {
    items: Vec<MaterialFontMeta>,
    categories: Vec<String>,
}

#[derive(Debug)]
pub enum LoadError {
    // IO(std::io::Error),
    Serde(String),
}

// impl std::convert::From<std::io::Error> for LoadError {
//     fn from(value: std::io::Error) -> Self {
//         Self::IO(value)
//     }
// }

impl Default for MaterialFontMetaList {
    fn default() -> Self {
        Self::empty()
    }
}

impl MaterialFontMetaList {
    pub fn empty() -> Self {
        Self {
            items: vec![],
            categories: vec![],
        }
    }

    pub fn items(&self) -> &Vec<MaterialFontMeta> {
        &self.items
    }

    pub fn get_item(&self, codepoint: u32) -> Option<&MaterialFontMeta> {
        self.items.iter().find(|item| item.codepoint == codepoint)
    }

    pub fn categories(&self) -> &Vec<String> {
        &self.categories
    }

    pub async fn load_from_bytes_fake_async(
        bytes: impl Into<std::borrow::Cow<'static, [u8]>>,
    ) -> Result<Self, LoadError> {
        Self::load_from_bytes(bytes)
    }

    pub fn load_from_bytes(
        bytes: impl Into<std::borrow::Cow<'static, [u8]>>,
    ) -> Result<Self, LoadError> {
        let value = match serde_json::from_slice::<serde_json::Value>(&bytes.into()) {
            Ok(value) => value,
            Err(e) => return Err(LoadError::Serde(e.to_string())),
        };

        Self::parse_json_value(value)
    }

    fn parse_json_value(value: serde_json::Value) -> Result<Self, LoadError> {
        let icons = value["icons"].clone();

        let items = match serde_json::from_value::<Vec<MaterialFontMeta>>(icons) {
            Ok(v) => v,
            Err(e) => return Err(LoadError::Serde(e.to_string())),
        };

        let categories = {
            let mut values = items
                .iter()
                .flat_map(|item| item.categories.clone())
                .collect::<Vec<String>>();
            values.sort();
            values.dedup();
            values
        };

        Ok(Self { items, categories })
    }
}
