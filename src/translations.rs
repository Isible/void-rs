use std::{collections::VecDeque, fs::File, io::Read};

use serde_json::Value;

use crate::LOGGER;

pub const DEFAULT_LANGUAGE: &str = "en-us";
pub const ERROR_MSG: &str = "Error";

pub struct Translator<'a> {
    json: Value,
    lang: &'a str,
}

impl<'a> Translator<'a> {
    pub fn new(lang: &'a str) -> Self {
        // TODO: proper error handling
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

        Self {
            json: parsed_data.unwrap(),
            lang,
        }
    }

    pub fn change_language(&mut self, new_lang: &'a str) {
        self.json = Self::new(new_lang).json;
        self.lang = new_lang;
    }

    pub fn load(&self, path: &str) -> &str {
        let mut paths: VecDeque<&str> = path.split(".").collect();
        let name = match paths.pop_back() {
            Some(name) => name,
            None => {
                LOGGER.error("Empty Translation path!".into());
                return ERROR_MSG.into();
            }
        };

        let mut value = self
            .json
            .get(match paths.front() {
                Some(val) => val,
                None => {
                    LOGGER.error(format!(
                        "The given translation path is too short and/or is not complete, given: {}",
                        path
                    ));
                    return ERROR_MSG.into();
                }
            })
            .unwrap();

        // remove the first section of the path
        paths.pop_front();

        for iter_path in paths {
            value = match value.get(iter_path) {
                Some(val) => val,
                None => {
                    LOGGER.info(format!(
                        "Failed to get path {} from translation path: {}",
                        iter_path, path
                    ));
                    return "Error";
                }
            };
        }

        match match value.get(name) {
            Some(val) => val,
            None => return ERROR_MSG.into(),
        }.as_str() {
            Some(ret) => ret,
            None => todo!(),
        }
    }

    pub fn lang(&self) -> &'a str {
        self.lang
    }
}
