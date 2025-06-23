use leptos::prelude::*;
use leptos::server;
use serde::{Deserialize, Serialize};

pub fn parse_ch_input(input: &str) -> Vec<&str> {
    match input {
        "" => Vec::new(),
        s if s.contains(',') => s.split(',').collect(),
        s if s.contains('，') => s.split('，').collect(),
        s => vec![s],
    }
}

pub fn parse_jap_input(input: &str) -> Vec<&str> {
    match input {
        "" => Vec::new(),
        s if s.contains(',') => s.split(',').collect(),
        s if s.contains('、') => s.split('、').collect(),
        s => vec![s],
    }
}

pub fn remove_whitespace(input: &str) -> String {
    input.chars().filter(|&c| !c.is_whitespace()).collect()
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DictionaryEntry {
    pub id: usize,
    pub hanzi: String,
    pub lecture: String,
    pub definition: String,
}

//
// At first there was only one struct dictionary but I do not know
// how to extract the correct dicionary from actix if both the Japanese
// and chinese dictionary have the same DataType so I've made two, at least for
// now, it lets me load the dictionaries when the server starts instead of on
// each request, improving the application speed.
//

#[derive(Serialize, Deserialize, Clone)]
pub struct JapaneseDictionary {
    entries: Vec<DictionaryEntry>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChineseDictionary {
    entries: Vec<DictionaryEntry>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
pub struct Flashcard {
    pub id: i32,
    pub front: RwSignal<String>,
    pub back: RwSignal<String>,
}

//
// At first there was only one struct dictionary and one load_dictionary
// function, but I do not know
// how to extract the correct dicionary from actix if both the Japanese
// and chinese dictionary have the same DataType so I've made two, at least for
// now, it lets me load the dictionaries when the server starts instead of on
// each request, improving the application speed.
//

pub async fn load_jap_dictionary(
    dictionary_path: String,
) -> Result<JapaneseDictionary, ServerFnError> {
    use leptos::logging::log;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    let file = File::open(dictionary_path).expect("Failed to open file");
    let reader = BufReader::with_capacity(64 * 1024, file);

    let mut entries = Vec::with_capacity(180000); // Could we overflow this?

    for (id, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(_) => {
                log!("Error reading line");
                continue; // Skip this line and continue with the next one
            }
        };

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue; // Skip empty lines
        }

        // Extract lecture, hanzi, and definitions
        if let Some(start_bracket) = trimmed_line.find('[') {
            if let Some(end_bracket) = trimmed_line.find(']') {
                // Extract parts more efficiently
                let hanzi_part = &trimmed_line[..start_bracket].trim();
                let lecture = &trimmed_line[start_bracket + 1..end_bracket];
                let definitions = &trimmed_line[end_bracket + 1..];

                // Get last word from hanzi part more efficiently
                let hanzi = hanzi_part.split_whitespace().last().unwrap_or("").trim();

                // Only allocate if we have valid data
                if !hanzi.is_empty() {
                    entries.push(DictionaryEntry {
                        id: id + 1,
                        hanzi: hanzi.to_string(), // to_string() is equivalent but more idiomatic
                        lecture: lecture.to_string(),
                        definition: definitions.trim().to_string(),
                    });
                }
            }
        }
    }

    entries.shrink_to_fit();
    // Return the populated dictionary
    Ok(JapaneseDictionary { entries })
}

pub async fn load_ch_dictionary(
    dictionary_path: String,
) -> Result<ChineseDictionary, ServerFnError> {
    use leptos::logging::log;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    let file = File::open(dictionary_path).expect("Failed to open file");
    let reader = BufReader::with_capacity(64 * 1024, file);

    let mut entries = Vec::with_capacity(140000); // Could we overflow this?

    for (id, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(_) => {
                log!("Error reading line");
                continue; // Skip this line and continue with the next one
            }
        };

        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue; // Skip empty lines
        }

        // Extract lecture, hanzi, and definitions
        if let Some(start_bracket) = trimmed_line.find('[') {
            if let Some(end_bracket) = trimmed_line.find(']') {
                // Extract parts more efficiently
                let hanzi_part = &trimmed_line[..start_bracket].trim();
                let lecture = &trimmed_line[start_bracket + 1..end_bracket];
                let definitions = &trimmed_line[end_bracket + 1..];

                // Get last word from hanzi part more efficiently
                let hanzi = hanzi_part.split_whitespace().last().unwrap_or("").trim();

                // Only allocate if we have valid data
                if !hanzi.is_empty() {
                    entries.push(DictionaryEntry {
                        id: id + 1,
                        hanzi: hanzi.to_string(), // to_string() is equivalent but more idiomatic
                        lecture: lecture.to_string(),
                        definition: definitions.trim().to_string(),
                    });
                }
            }
        }
    }

    entries.shrink_to_fit();
    // Return the populated dictionary
    Ok(ChineseDictionary { entries })
}

#[server(SearchDictionary, "/searchdictionary")]
pub async fn search_dictionary(
    chars_string: String,
    is_ch: bool,
) -> Result<Vec<Flashcard>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use std::collections::{HashMap, HashSet};

    let chars_array = if is_ch {
        parse_ch_input(&chars_string)
    } else {
        parse_jap_input(&chars_string)
    };

    if chars_array.is_empty() {
        return Ok(Vec::new());
    }

    let mut flashcards = Vec::with_capacity(chars_array.len() * 2);
    let mut seen_chars = HashSet::with_capacity(chars_array.len());
    let mut id_counter = 1;

    let search_chars: HashSet<&str> = chars_array.iter().map(|s| s.trim()).collect();

    if is_ch {
        let ch_dictionary: Data<ChineseDictionary> = extract().await?;

        // index only for characters we're searching for
        let mut char_to_entries: HashMap<&str, Vec<&DictionaryEntry>> =
            HashMap::with_capacity(search_chars.len());

        for entry in &ch_dictionary.entries {
            let hanzi = entry.hanzi.trim();
            if search_chars.contains(hanzi) {
                char_to_entries.entry(hanzi).or_default().push(entry);
            }
        }

        // loop in original order, avoiding duplicates
        for &ch in &chars_array {
            let trimmed_ch = ch.trim();
            if seen_chars.insert(trimmed_ch) {
                if let Some(entries) = char_to_entries.get(trimmed_ch) {
                    for entry in entries {
                        flashcards.push(Flashcard {
                            id: id_counter,
                            front: RwSignal::new(entry.hanzi.clone()),
                            back: RwSignal::new(format!("{} {}", entry.lecture, entry.definition)),
                        });
                        id_counter += 1;
                    }
                } else {
                    flashcards.push(Flashcard {
                        id: id_counter,
                        front: RwSignal::new(trimmed_ch.to_string()),
                        back: RwSignal::new("NOT FOUND".to_string()),
                    });
                    id_counter += 1;
                }
            }
        }
    } else {
        let jap_dictionary: Data<JapaneseDictionary> = extract().await?;

        // index only for characters we're searching for
        let mut char_to_entries: HashMap<&str, Vec<&DictionaryEntry>> =
            HashMap::with_capacity(search_chars.len());

        for entry in &jap_dictionary.entries {
            let hanzi = entry.hanzi.trim();
            if search_chars.contains(hanzi) {
                char_to_entries.entry(hanzi).or_default().push(entry);
            }
        }

        // loop in original order, avoiding duplicates
        for &ch in &chars_array {
            let trimmed_ch = ch.trim();
            if seen_chars.insert(trimmed_ch) {
                if let Some(entries) = char_to_entries.get(trimmed_ch) {
                    for entry in entries {
                        flashcards.push(Flashcard {
                            id: id_counter,
                            front: RwSignal::new(entry.hanzi.clone()),
                            back: RwSignal::new(format!("{} {}", entry.lecture, entry.definition)),
                        });
                        id_counter += 1;
                    }
                } else {
                    flashcards.push(Flashcard {
                        id: id_counter,
                        front: RwSignal::new(trimmed_ch.to_string()),
                        back: RwSignal::new("NOT FOUND".to_string()),
                    });
                    id_counter += 1;
                }
            }
        }
    }

    Ok(flashcards)
}

pub fn create_vaia_import_string(flashcards: &Vec<Flashcard>) -> String {
    let mut result = String::new();

    for (i, entry) in flashcards.iter().enumerate() {
        result = result + &entry.front.get_untracked() + "/#*#/" + &entry.back.get_untracked();

        if i + 1 != flashcards.len() {
            result.push_str("\n\\#\n");
        }
    }

    result
}

pub fn create_quizlet_import_string(flashcards: &Vec<Flashcard>) -> String {
    let mut result = String::new();

    for (i, entry) in flashcards.iter().enumerate() {
        result = result + &entry.front.get_untracked() + "\t" + &entry.back.get_untracked();

        if i + 1 != flashcards.len() {
            result.push_str("\n");
        }
    }

    result
}

pub fn create_anki_hex_file(flashcards: &Vec<Flashcard>) -> String {
    let mut result = String::new();
    result = result + "#separator:tab\n";
    result = result + "#html:false\n";

    for (i, entry) in flashcards.iter().enumerate() {
        result = result + &entry.front.get_untracked() + "\t" + &entry.back.get_untracked();

        if i + 1 != flashcards.len() {
            result.push_str("\n");
        }
    }

    encode_hex(&result)
}

fn encode_hex(input: &str) -> String {
    let mut result = String::new();
    for byte in input.bytes() {
        // Encode special characters
        if byte == b' ' {
            result.push_str("%20");
        } else if byte == b'\n' {
            result.push_str("%0A");
        } else {
            result.push_str(&format!("%{:02X}", byte));
        }
    }
    result
}
