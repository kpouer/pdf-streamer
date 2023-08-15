use std::collections::BTreeMap;

#[derive(Default)]
pub struct Context {
    pub text: String,
    pub current_encoding: Option<String>,
    pub encodings: BTreeMap<Vec<u8>, String>,
}