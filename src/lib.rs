mod text_context;
mod operator;

use std::collections::{BTreeMap, HashMap};
use lopdf::content::Content;
use lopdf::{Document, Object, ObjectId};
use operator::set_font_and_size;
use crate::operator::{end_text, move_text, next_line, Operator, set_graphic_state_params, show_text, show_text_adjusted};
use crate::text_context::Context;

pub fn extract_text(doc: &Document) -> String {
    let mut text_context = Context::default();
    stream_document(doc, &mut text_context);
    text_context.text
}

pub fn extract_text_from_page(doc: &Document, page: u32) -> String {
    extract_text_from_pages(doc, &[page])
}

pub fn extract_text_from_pages(doc: &Document, pages: &[u32]) -> String {
    let mut text_context = Context::default();
    stream_pages(doc, &mut text_context, pages);
    text_context.text
}

pub fn stream_document(doc: &Document, text_context: &mut Context) {
    let pages = doc.get_pages();
    let page_numbers = pages.keys().map(|k| *k).collect::<Vec<u32>>();
    let page_numbers = page_numbers.as_slice();
    do_stream_pages(doc, text_context, get_default_operators(), &pages, page_numbers);
}

pub fn stream_pages(doc: &Document, text_context: &mut Context, page_numbers: &[u32]) {
    let pages = doc.get_pages();
    let  operators = get_default_operators();
    do_stream_pages(doc, text_context, operators, &pages, page_numbers);
}

fn get_default_operators() -> HashMap<String, Box<dyn Operator>> {
    let mut operators : HashMap<String, Box<dyn Operator>> = HashMap::new();
    operators.insert(end_text::OP.to_string(), Box::new(end_text::EndText{}));
    operators.insert(move_text::OP.to_string(), Box::new(move_text::MoveText{}));
    operators.insert(next_line::OP.to_string(), Box::new(next_line::NextLine{}));
    operators.insert(set_font_and_size::OP.to_string(), Box::new(set_font_and_size::SetFontAndSize{}));
    operators.insert(set_graphic_state_params::OP.to_string(), Box::new(set_graphic_state_params::SetGraphicStateParams{}));
    operators.insert(show_text::OP.to_string(), Box::new(show_text::ShowText{}));
    operators.insert(show_text_adjusted::OP.to_string(), Box::new(show_text_adjusted::ShowText{}));
    operators
}

fn do_stream_pages(doc: &Document, text_context: &mut Context, operators: HashMap<String, Box<dyn Operator>>, pages: &BTreeMap<u32, ObjectId>, page_numbers: &[u32]) {
    for page_number in page_numbers {
        let page_id = pages.get(page_number);
        if page_id.is_some() {
            let page_id = *page_id.unwrap();
            begin_page(text_context, doc, &page_id);
            let content_data = doc.get_page_content(page_id).unwrap();
            let content = Content::decode(&content_data).unwrap();
            for operation in &content.operations {
                let op: &str = operation.operator.as_ref();
                let operator = operators.get(op);
                if operator.is_some() {
                    let operator: &Box<dyn Operator> = operator.unwrap();
                    let operator = operator.as_ref();
                    operator.process(text_context, operation);
                }
//                operator.process(doc, operation);
            }
        }
    }
}

fn begin_page(text_context: &mut Context, doc: &Document, page_id: &ObjectId) {
    let fonts = doc.get_page_fonts(*page_id);
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