use std::collections::BTreeMap;

pub struct Context {
    pub text: String,
    pub current_encoding: Option<String>,
    pub encodings: BTreeMap<Vec<u8>, String>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            text: String::new(),
            current_encoding: None,
            encodings: BTreeMap::new(),
        }
    }
}