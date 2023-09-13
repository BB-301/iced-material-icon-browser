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

    pub fn categories(&self) -> &Vec<String> {
        &self.categories
    }

    pub fn tags(&self) -> &Vec<String> {
        &self.tags
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

    pub fn matches_name(&self, name: &String) -> bool {
        &self.name == name
    }

    pub fn matches_any(&self, value: &String) -> bool {
        if self.matches_name(value) {
            return true;
        }
        if self.contains_category(value) {
            return true;
        }
        if self.contains_tag(value) {
            return true;
        }
        if self.matches_hex_codepoint(value) {
            return true;
        }
        false
    }

    pub fn matches_hex_codepoint(&self, codepoint: &String) -> bool {
        let hex_codepoint = format!("{:08x}", self.codepoint);
        hex_codepoint.ends_with(codepoint)
    }
}

#[derive(Clone, Debug)]
pub struct MaterialFontMetaList {
    items: Vec<MaterialFontMeta>,
    categories: Vec<String>,
    tags: Vec<String>,
}

#[derive(Debug)]
pub enum LoadError {
    IO(std::io::Error),
    Serde(String),
}

impl std::convert::From<std::io::Error> for LoadError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

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
            tags: vec![],
        }
    }

    pub fn items(&self) -> &Vec<MaterialFontMeta> {
        &self.items
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn categories(&self) -> &Vec<String> {
        &self.categories
    }

    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub async fn load_async(file_path: &str) -> Result<Self, LoadError> {
        let contents = match tokio::fs::read_to_string(file_path).await {
            Ok(contents) => contents,
            Err(e) => return Err(LoadError::IO(e)),
        };

        Self::parse_json_contents(contents)
    }

    pub fn load_from_bytes() -> Result<Self, LoadError> {
        panic!("Please implement me!");
    }

    fn parse_json_contents(contents: String) -> Result<Self, LoadError> {
        let value = match serde_json::from_str::<serde_json::Value>(&contents) {
            Ok(value) => value,
            Err(e) => return Err(LoadError::Serde(e.to_string())),
        };

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

        let tags = {
            let mut values = items
                .iter()
                .flat_map(|item| item.tags.clone())
                .collect::<Vec<String>>();
            values.sort();
            values.dedup();
            values
        };

        Ok(Self {
            items,
            categories,
            tags,
        })
    }

    pub fn load(file_path: &str) -> Result<Self, LoadError> {
        let contents = std::fs::read_to_string(file_path)?;

        Self::parse_json_contents(contents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const META_FILE_PATH: &'static str = "./resources/material-icons-meta.json";

    #[test]
    fn test_load_method() {
        let list = MaterialFontMetaList::load(META_FILE_PATH).unwrap();
        println!("{} items loaded", list.items.len());

        println!("Categories: {:?}", list.categories);
        println!("Tags: {:?}", list.tags);
    }

    #[test]
    fn test_font_meta_deserialize_array() {
        let contents = std::fs::read_to_string(META_FILE_PATH).unwrap();
        let value: serde_json::Value = serde_json::from_str(&contents).unwrap();
        let icons = value["icons"].clone();
        let items: Vec<MaterialFontMeta> = serde_json::from_value(icons).unwrap();
        assert!(items.len() > 0);
        println!("{} items found", items.len());
    }

    #[test]
    fn test_font_meta_deserialize() {
        let json = r#"
        {
            "name": "10k",
            "version": 10,
            "popularity": 1161,
            "codepoint": 59729,
            "unsupported_families": [],
            "categories": [
              "av"
            ],
            "tags": [
              "10000",
              "10K",
              "alphabet",
              "character",
              "digit",
              "display",
              "font",
              "letter",
              "number",
              "pixel",
              "pixels",
              "resolution",
              "symbol",
              "text",
              "type",
              "video"
            ],
            "sizes_px": [
              24
            ]
          }
        "#;
        match serde_json::from_str::<MaterialFontMeta>(json) {
            Ok(meta) => println!("{:#?}", meta),
            Err(e) => panic!("{:?}", e),
        }
    }
}
