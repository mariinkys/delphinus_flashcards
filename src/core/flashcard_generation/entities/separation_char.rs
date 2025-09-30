use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SeparationChar {
    #[default]
    Normal,
    Japanese,
    Chinese,
}

impl Display for SeparationChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            SeparationChar::Normal => write!(f, "Normal (,)"),
            SeparationChar::Japanese => write!(f, "Japanese (、)"),
            SeparationChar::Chinese => write!(f, "Chinese (，)"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseSeparationCharError {
    pub invalid_value: String,
}

impl Display for ParseSeparationCharError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Char theme: '{}'", self.invalid_value)
    }
}

impl std::error::Error for ParseSeparationCharError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl FromStr for SeparationChar {
    type Err = ParseSeparationCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "Normal (,)" => Ok(SeparationChar::Normal),
            "Japanese (、)" => Ok(SeparationChar::Japanese),
            "Chinese (，)" => Ok(SeparationChar::Chinese),
            _ => Err(ParseSeparationCharError {
                invalid_value: s.to_string(),
            }),
        }
    }
}

impl SeparationChar {
    pub const ALL: &'static [Self] = &[Self::Normal, Self::Japanese, Self::Chinese];

    pub fn get_char(&self) -> &'static str {
        match &self {
            SeparationChar::Normal => ",",
            SeparationChar::Japanese => "、",
            SeparationChar::Chinese => "，",
        }
    }
}
