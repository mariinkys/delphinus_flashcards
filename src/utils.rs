use leptos::{server, RwSignal, ServerFnError, SignalGetUntracked};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

pub fn parse_ch_input(input: &str) -> Vec<&str> {
    let mut chars_array: Vec<&str> = Vec::new();
    if input.contains(",") {
        chars_array = input.split(",").collect();
    } else if input.contains("，") {
        chars_array = input.split("，").collect();
    } else if input.is_empty() {
        return chars_array; // Return empty vector
    } else {
        chars_array.push(input);
    }
    chars_array
}

pub fn parse_jap_input(input: &str) -> Vec<&str> {
    let mut chars_array: Vec<&str> = Vec::new();
    if input.contains(",") {
        chars_array = input.split(",").collect();
    } else if input.contains("、") {
        chars_array = input.split("、").collect();
    } else if input.is_empty() {
        return chars_array; // Return empty vector
    } else {
        chars_array.push(input);
    }
    chars_array
}

pub fn remove_whitespace(input: &str) -> String {
    input.chars().filter(|&c| !c.is_whitespace()).collect()
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DictionaryEntry {
    pub id: u32,
    pub hanzi: String,
    pub lecture: String,
    pub definition: String,
}

#[derive(Serialize, Deserialize)]
pub struct Dictionary {
    entries: Vec<DictionaryEntry>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Flashcard {
    pub id: u32,
    pub front: RwSignal<String>,
    pub back: RwSignal<String>,
}

#[server(LoadDictionary, "/loaddictionary")]
pub async fn load_dictionary() -> Result<Dictionary, ServerFnError> {
    use leptos::logging::log;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    let file = File::open("dictionaries/ch/cedict_ts.u8").expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut dict = Dictionary {
        entries: Vec::new(),
    };

    let mut count = 0;

    for line in reader.lines() {
        count = count + 1;
        let line = match line {
            Ok(line) => line,
            Err(_e) => {
                log!("Error reading line");
                continue; // Skip this line and continue with the next one
            }
        };

        let trimmed_line = line.trim();

        // Extract lecture, hanzi, and definitions
        let mut parts = trimmed_line.splitn(3, |c| c == '[' || c == ']');
        let mut hanzi = parts.next().unwrap_or("").trim();
        let lecture = parts.next().unwrap_or("").trim();
        let definitions = parts.next().unwrap_or("").trim();

        hanzi = hanzi.split(" ").last().unwrap_or("").trim();

        // Add entry to dictionary
        dict.entries.push(DictionaryEntry {
            id: count,
            hanzi: String::from(hanzi),
            lecture: String::from(lecture),
            definition: String::from(definitions),
        });
    }

    // Return the populated dictionary
    Ok(dict)
}

pub fn search_dictionary(dict: &Dictionary, chars_array: Vec<&str>) -> Vec<Flashcard> {
    let mut res_array = Vec::new();

    // For each entry in the dictionary
    for entry in &dict.entries {
        // For each character in the chars_array
        for &ch in &chars_array {
            // If the character matches the kanji in the entry, append the entry to the result array
            if ch.trim() == entry.hanzi.trim() {
                let fc: Flashcard = Flashcard {
                    id: entry.id,
                    front: entry.hanzi.to_string().into(),
                    back: format!("{} {}", entry.lecture, entry.definition).into(),
                };
                res_array.push(fc);
            }
        }
    }

    res_array
}

pub fn create_import_string(flashcards: Vec<Flashcard>) -> String {
    let mut result = String::new();

    for (i, entry) in flashcards.iter().enumerate() {
        write!(
            &mut result,
            "{}/#*#/{}/",
            entry.front.get_untracked(),
            entry.back.get_untracked()
        )
        .expect("Error writing to string");

        if i + 1 != flashcards.len() {
            result.push_str("\\#\n");
        }
    }

    result
}
