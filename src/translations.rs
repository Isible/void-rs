use std::{fs::File, fmt::format, io::Read, collections::VecDeque};

use serde_json::Value;

pub const DEFAULT_LANGUAGE: &str = "en-us";

pub struct Translator<'a> {
    json: Value,
    lang: &'a str,
}

impl<'a> Translator<'a> {
    pub fn new(lang: &'a str) -> Self {
        // Read the file content into a string
        let mut file_content = String::new();
        match File::open(format!("resources/lang/{}.json", lang)) {
            Ok(mut file) => {
                if let Err(e) = file.read_to_string(&mut file_content) {
                    panic!("Error reading file: {}", e);
                }
            }
            Err(e) => {
                panic!("Error opening file: {}", e);
            }
        }

        // Parse JSON data into a serde_json::Value
        let parsed_data: Result<Value, serde_json::Error> = serde_json::from_str(&file_content);

        Self { json: parsed_data.unwrap(), lang }
    }

    pub fn change_language(&mut self, new_lang: &'a str) {
        self.json = Self::new(new_lang).json;
        self.lang = new_lang;
    }

    pub fn load(&self, path: &str) -> &str {
        let mut paths: VecDeque<&str> = path.split(".").collect();
        let name = paths.pop_back();
        let mut value = self.json.get(paths.front().unwrap()).unwrap();
        paths.pop_front();
        for path in paths {
            value = value.get(path).unwrap();
        }
        value.get(name.unwrap()).unwrap().as_str().unwrap()
    }

    pub fn lang(&self) -> &'a str {
        self.lang
    }
}
