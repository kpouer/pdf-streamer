mod text_context;
pub mod rules;

use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;
use lopdf::content::Content;
use lopdf::{Document, Object, ObjectId};
use crate::DebugMode::{All, Disabled, Unmatched};
use crate::rules::operator::Operator;
use crate::text_context::Context;

pub struct Options {
    pub doc: Document,
    pub page: u32,
    pub debug_mode: DebugMode,
    pub rules: HashMap<String, Box<dyn Operator>>
}

impl Options {
    pub fn new(doc: Document) -> Self {
        Options {
            doc,
            page: 0,
            debug_mode: Disabled,
            rules: rules::default_rules()
        }
    }
}


#[derive(PartialEq)]
pub enum DebugMode {
    Disabled,
    Unmatched,
    All
}

impl Default for DebugMode {
    fn default() -> Self {
        Disabled
    }
}

impl FromStr for DebugMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Unmatched),
            "2" => Ok(All),
            _ => Ok(Disabled)
        }
    }
}

pub fn extract_text(options: &Options) -> String {
    let mut text_context = Context::default();
    stream_document(options, &mut text_context);
    text_context.text
}

pub fn extract_text_from_page(options: &Options, page: u32) -> String {
    extract_text_from_pages(options, &[page])
}

pub fn extract_text_from_pages(options: &Options, pages: &[u32]) -> String {
    let mut text_context = Context::default();
    stream_pages(options, &mut text_context, pages);
    text_context.text
}

pub fn stream_document(options: &Options, text_context: &mut Context) {
    let pages = options.doc.get_pages();
    let page_numbers = pages.keys().cloned().collect::<Vec<u32>>();
    let page_numbers = page_numbers.as_slice();
    do_stream_pages(options, text_context, &pages, page_numbers);
}

pub fn stream_pages(options: &Options, text_context: &mut Context, page_numbers: &[u32]) {
    let pages = options.doc.get_pages();
    do_stream_pages(options, text_context, &pages, page_numbers);
}

fn do_stream_pages(options: &Options,
                   text_context: &mut Context,
                   pages: &BTreeMap<u32, ObjectId>,
                   page_numbers: &[u32]) {
    for page_number in page_numbers {
        let page_id = pages.get(page_number);
        match page_id {
            None => {}
            Some(page_id) => {
                let page_id = *page_id;
                begin_page(options, text_context, &page_id);
                let content_data = options.doc.get_page_content(page_id).unwrap();
                let content = Content::decode(&content_data).unwrap();
                for operation in &content.operations {
                    let op: &str = operation.operator.as_ref();
                    let operator = options.rules.get(op);
                    match operator {
                        None => {
                            if options.debug_mode != Disabled {
                                text_context.text.push_str(format!("<{}>", op).as_str());
                            }
                        }
                        Some(operator) => {
                            if options.debug_mode == All {
                                text_context.text.push_str(format!("<{}>", op).as_str());
                            }
                            let operator = operator.as_ref();
                            operator.process(text_context, operation);
                            if options.debug_mode == All {
                                text_context.text.push_str(format!("</{}>", op).as_str());
                            }
                        }
                    }
                }
            }
        }
    }
}

fn begin_page(options: &Options, text_context: &mut Context, page_id: &ObjectId) {
    let fonts = options.doc.get_page_fonts(*page_id);
    text_context.encodings = fonts
        .into_iter()
        .map(|(name, font)| (name, font.get_font_encoding().to_string()))
        .collect::<BTreeMap<Vec<u8>, String>>();
}

pub fn collect_text(text: &mut String, encoding: &Option<String>, operands: &[Object]) {
    for operand in operands.iter() {
        match *operand {
            Object::String(ref bytes, _) => {
                let encoding = encoding.as_ref().map(|s| s.as_str());
                let decoded_text = Document::decode_text(encoding, bytes);
                text.push_str(&decoded_text);
            }
            Object::Array(ref arr) => {
                collect_text(text, encoding, arr);
                text.push(' ');
            }
            Object::Integer(i) => {
                if i < -100 {
                    text.push(' ');
                }
            }
            _ => {}
        }
    }
}