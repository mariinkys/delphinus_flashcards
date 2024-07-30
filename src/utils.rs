use leptos::prelude::{GetUntracked, RwSignal, ServerFnError};
use leptos::server;
use serde::{Deserialize, Serialize};

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
    pub id: u32,
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
    let reader = BufReader::new(file);
    let mut dict = JapaneseDictionary {
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

pub async fn load_ch_dictionary(
    dictionary_path: String,
) -> Result<ChineseDictionary, ServerFnError> {
    use leptos::logging::log;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    let file = File::open(dictionary_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut dict = ChineseDictionary {
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

    let jap_dictionary: Data<JapaneseDictionary> = extract().await?;
    let ch_dictionary: Data<ChineseDictionary> = extract().await?;

    let mut res_array = Vec::new();
    let mut found_chars = HashSet::new(); // Keep track of found characters
    let mut count = 1;

    let mut flashcards_map: HashMap<&str, Vec<Flashcard>> = HashMap::new();

    if is_ch {
        // For each entry in the dictionary
        for entry in &ch_dictionary.entries {
            // For each character in the chars_array
            for &ch in &chars_array {
                // If the character matches the kanji in the entry, append the entry to the result array
                if ch.trim() == entry.hanzi.trim() {
                    let fc: Flashcard = Flashcard {
                        id: count,
                        front: RwSignal::new(entry.hanzi.to_string()),
                        back: RwSignal::new(format!("{} {}", entry.lecture, entry.definition)),
                    };
                    count = count + 1;
                    flashcards_map.entry(ch).or_insert(Vec::new()).push(fc);
                    found_chars.insert(ch);
                }
            }
        }
    } else {
        // For each entry in the dictionary
        for entry in &jap_dictionary.entries {
            // For each character in the chars_array
            for &ch in &chars_array {
                // If the character matches the kanji in the entry, append the entry to the result array
                if ch.trim() == entry.hanzi.trim() {
                    let fc: Flashcard = Flashcard {
                        id: count,
                        front: RwSignal::new(entry.hanzi.to_string()),
                        back: RwSignal::new(format!("{} {}", entry.lecture, entry.definition)),
                    };
                    count = count + 1;
                    flashcards_map.entry(ch).or_insert(Vec::new()).push(fc);
                    found_chars.insert(ch);
                }
            }
        }
    }

    count = count + 1;
    // Check for characters not found in the dictionary
    for &ch in &chars_array {
        if !found_chars.contains(&ch.trim()) {
            let fc: Flashcard = Flashcard {
                id: count,
                front: RwSignal::new(ch.to_string()),
                back: RwSignal::new("NOT FOUND".to_string()),
            };
            count = count + 1;
            flashcards_map.entry(ch).or_insert(Vec::new()).push(fc);
        }
    }

    // Iterate over chars_array to maintain the order and push Flashcards from the HashMap into res_array
    let mut seen_flashcards = HashSet::new();
    for ch in chars_array {
        if let Some(flashcards) = flashcards_map.get(ch) {
            for flashcard in flashcards {
                if seen_flashcards.insert(flashcard.clone()) {
                    // If flashcard is not already in seen_flashcards, insert it into res_array
                    res_array.push(flashcard.clone());
                }
            }
        }
    }

    Ok(res_array)
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
