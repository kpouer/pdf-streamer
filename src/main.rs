use std::collections::HashMap;
use std::str::FromStr;
use lopdf::Document;
use pdf_streamer::{Options, rules, DebugMode};

#[derive(Default)]
struct Params {
    filename: String,
    page: u32,
    debug_mode: DebugMode,
    no_rules: bool,
    rules_file: Option<String>,
}

fn main() {
    if let Ok(params) = parse_options() {
        let doc = Document::load(params.filename).unwrap();
        let rules;
        if params.no_rules {
            rules = params.rules_file
                .map_or_else(|| HashMap::new(),
                             |rule_file| rules::custom_rules(rule_file));
        } else {
            rules = params.rules_file
                .map_or_else(|| rules::default_rules(),
                             |rule_file| rules::default_and_custom_rules(rule_file));
        }
        let options = Options {
            doc,
            page: params.page,
            debug_mode: params.debug_mode,
            rules,
        };
        let text = pdf_streamer::extract_text_from_page(&options, params.page);
        println!("{}", text);
    }
}

fn parse_options() -> Result<Params, String> {
    let args: Vec<String> = std::env::args().collect();
    let mut params = Params::default();

    let mut i = 1;
    while i < args.len() {
        let current_param = &args[i];
        if current_param.starts_with("-") {
            if current_param == "-p" || current_param == "--page" {
                params.page = args[i + 1].parse::<u32>().unwrap();
                i += 2;
            } else if current_param == "-d" || current_param == "--debug" {
                let mode = args[i + 1].clone();
                params.debug_mode = DebugMode::from_str(mode.as_str()).unwrap();
                i += 1;
            } else if current_param == "-n" || current_param == "-no-rule" {
                params.no_rules = true;
                i += 1;
            } else if current_param == "-r" || current_param == "-rules" {
                params.rules_file = Some(args[i + 1].clone());
                i += 2;
            }
        } else {
            params.filename = args[i].clone();
            i += 1;
        }
    }

    if params.filename == "" {
        return Err("PDF file should not be empty".to_string());
    }
    return Ok(params);
}
