pub mod nothing_action;
pub mod custom_string_action;
pub mod optional_space;
pub mod custom_char_action;
pub mod set_font_and_size;
pub mod extract_text;
pub mod compound_action;

use std::str::FromStr;
use crate::rules::action::Action::CustomText;

#[derive(Debug)]
pub(crate) enum Action {
    Nothing,
    CustomText
}

impl FromStr for Action {
    type Err = ();

    fn from_str(input: &str) -> Result<Action, Self::Err> {
        let input = input.trim().to_lowercase();
        let input = input.as_str();
        match input {
            "<nothing>" => Ok(Action::Nothing),
            _ => Ok(CustomText),
        }
    }
}