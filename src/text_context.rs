use std::collections::BTreeMap;

pub struct Context {
    pub text: String,
    pub current_encoding: Option<String>,
    pub encodings: BTreeMap<Vec<u8>, String>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            text: String::new(),
            current_encoding: None,
            encodings: BTreeMap::new(),
        }
    }
}