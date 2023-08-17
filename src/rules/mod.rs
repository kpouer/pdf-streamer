pub(crate) mod action;
pub(crate) mod operator;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;
use crate::rules::action::{Action, custom_char_action, custom_string_action, nothing_action, set_font_and_size, extract_text, compound_action};
use crate::rules::action::optional_space::OptionalSpace;
use crate::rules::operator::{END_TEXT, MOVE_TEXT, MOVE_TEXT_SET_LEADING, NEXT_LINE, Operator, SET_FONT_AND_SIZE, SET_GRAPHICS_STATE_PARAMS, SHOW_TEXT, SHOW_TEXT_ADJUSTED};

pub fn default_rules() -> HashMap<String, Box<dyn Operator>> {
    let mut operators : HashMap<String, Box<dyn Operator>> = HashMap::new();
    operators.insert(END_TEXT.to_string(), Box::new(OptionalSpace{}));
    operators.insert(MOVE_TEXT.to_string(), Box::new(custom_char_action::CustomChar {char: ' '}));
    operators.insert(MOVE_TEXT_SET_LEADING.to_string(), Box::new(custom_char_action::CustomChar {char: ' '}));
    operators.insert(NEXT_LINE.to_string(), Box::new(custom_char_action::CustomChar{char: '\n'}));
    operators.insert(SET_FONT_AND_SIZE.to_string(), Box::new(set_font_and_size::SetFontAndSize{}));
    operators.insert(SET_GRAPHICS_STATE_PARAMS.to_string(), Box::new(custom_char_action::CustomChar{char: '\n'}));
    operators.insert(SHOW_TEXT.to_string(), Box::new(extract_text::ExtractText {}));

    {
        let extract_text = Box::new(extract_text::ExtractText {});
        let space = Box::new(custom_char_action::CustomChar {char: ' '});
        operators.insert(SHOW_TEXT_ADJUSTED.to_string(), Box::new(compound_action::CompoundAction{actions: vec![extract_text, space]}));
    }
    operators
}

pub fn custom_rules<P>(file: P) -> HashMap<String, Box<dyn Operator>> where P: AsRef<Path> {
    let mut rules = HashMap::new();
    load_custom(file, &mut rules);
    rules
}

pub fn default_and_custom_rules<P>(file: P) -> HashMap<String, Box<dyn Operator>> where P: AsRef<Path> {
    let mut rules = default_rules();
    load_custom(file, &mut rules);
    rules
}

fn get_operator_action(action_name: &str) -> Box<dyn Operator> {
    let action = Action::from_str(action_name);
    if let Ok(action) = action {
        let action = get_action(&action);
        if action.is_some() {
            return action.unwrap();
        }
    }
    Box::new(custom_string_action::CustomString { text: action_name.to_string() })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_action(action: &Action) -> Option<Box<dyn Operator>> {
    match action {
        Action::Nothing => {
            Some(Box::new(nothing_action::Nothing {}))
        }
        _ => None
    }
}

fn load_custom<P>(file: P, operators: &mut HashMap<String, Box<dyn Operator>>) where P: AsRef<Path> {
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            match line {
                Ok(line) => {
                    let index_of_colon = line.find(":");
                    match index_of_colon {
                        None => {}
                        Some(index_of_colon) => {
                            let operator = &line[0..index_of_colon];
                            let action = &line[index_of_colon + 1..];
                            let operator_action = get_operator_action(action);
                            operators.insert(operator.to_string(), operator_action);
                        }
                    }
                }
                Err(_) => {}
            }
        }
    }
}
