use leptos::prelude::*;
use leptos::server;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use memmap2::Mmap;
#[cfg(feature = "ssr")]
use std::collections::HashMap;
#[cfg(feature = "ssr")]
use std::sync::Arc;

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

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct JapaneseDictionary {
    mmap: Arc<Mmap>,
    line_offsets: Vec<(usize, usize)>, // (start, end) positions for each line
}

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct ChineseDictionary {
    mmap: Arc<Mmap>,
    line_offsets: Vec<(usize, usize)>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
pub struct Flashcard {
    pub id: i32,
    pub front: RwSignal<String>,
    pub back: RwSignal<String>,
}

#[cfg(feature = "ssr")]
impl ChineseDictionary {
    /// Init the chinese dictionary
    pub async fn init(dictionary_path: String) -> Result<ChineseDictionary, ServerFnError> {
        use leptos::logging::log;

        let file = std::fs::File::open(dictionary_path).expect("Failed to open file");
        let mmap = unsafe { memmap2::Mmap::map(&file).expect("Failed to create memory map") };
        let mmap = std::sync::Arc::new(mmap);

        // Build index of line positions instead of parsing all entries
        let mut line_offsets = Vec::with_capacity(140000);
        let mut start = 0;

        for (pos, &byte) in mmap.iter().enumerate() {
            if byte == b'\n' {
                line_offsets.push((start, pos));
                start = pos + 1;
            }
        }

        // Handle last line if file doesn't end with newline
        if start < mmap.len() {
            line_offsets.push((start, mmap.len()));
        }

        line_offsets.shrink_to_fit();
        log!("Indexed {} lines in chinese dictionary", line_offsets.len());

        Ok(ChineseDictionary { mmap, line_offsets })
    }

    /// Get all entries of the dictionary
    pub fn entries(&self) -> impl Iterator<Item = DictionaryEntry> + '_ {
        self.line_offsets
            .iter()
            .enumerate()
            .filter_map(|(id, &(start, end))| {
                let line_bytes = &self.mmap[start..end];
                if let Ok(line) = std::str::from_utf8(line_bytes) {
                    self.parse_line(line, id)
                } else {
                    None
                }
            })
    }

    /// Search without loading all entries
    pub fn search(&self, query: &str) -> Vec<DictionaryEntry> {
        self.entries()
            .filter(|entry| entry.hanzi.contains(query) || entry.definition.contains(query))
            .collect()
    }

    /// Find entries for specific characters
    pub fn find_entries_for_chars(
        &self,
        search_chars: &std::collections::HashSet<&str>,
    ) -> HashMap<String, Vec<DictionaryEntry>> {
        let mut char_to_entries: HashMap<String, Vec<DictionaryEntry>> =
            HashMap::with_capacity(search_chars.len());

        for (id, &(start, end)) in self.line_offsets.iter().enumerate() {
            let line_bytes = &self.mmap[start..end];
            if let Ok(line) = std::str::from_utf8(line_bytes) {
                let contains_search_char = search_chars.iter().any(|&ch| line.contains(ch));
                if !contains_search_char {
                    continue;
                }

                if let Some(entry) = self.parse_line(line, id) {
                    let hanzi = entry.hanzi.trim();
                    if search_chars.contains(hanzi) {
                        char_to_entries
                            .entry(hanzi.to_string())
                            .or_default()
                            .push(entry);
                    }
                }
            }
        }

        char_to_entries
    }

    fn parse_line(&self, line: &str, id: usize) -> Option<DictionaryEntry> {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            return None;
        }

        if let Some(start_bracket) = trimmed_line.find('[') {
            if let Some(end_bracket) = trimmed_line.find(']') {
                let hanzi_part = &trimmed_line[..start_bracket].trim();
                let lecture = &trimmed_line[start_bracket + 1..end_bracket];
                let definitions = &trimmed_line[end_bracket + 1..];
                let hanzi = hanzi_part.split_whitespace().last().unwrap_or("").trim();

                if !hanzi.is_empty() {
                    return Some(DictionaryEntry {
                        id: id + 1,
                        hanzi: hanzi.to_string(),
                        lecture: lecture.to_string(),
                        definition: definitions.trim().to_string(),
                    });
                }
            }
        }
        None
    }
}

#[cfg(feature = "ssr")]
impl JapaneseDictionary {
    /// Init the chinese dictionary
    pub async fn init(dictionary_path: String) -> Result<JapaneseDictionary, ServerFnError> {
        use leptos::logging::log;

        let file = std::fs::File::open(dictionary_path).expect("Failed to open file");
        let mmap = unsafe { memmap2::Mmap::map(&file).expect("Failed to create memory map") };
        let mmap = std::sync::Arc::new(mmap);

        // Build index of line positions instead of parsing all entries
        let mut line_offsets = Vec::with_capacity(140000);
        let mut start = 0;

        for (pos, &byte) in mmap.iter().enumerate() {
            if byte == b'\n' {
                line_offsets.push((start, pos));
                start = pos + 1;
            }
        }

        // Handle last line if file doesn't end with newline
        if start < mmap.len() {
            line_offsets.push((start, mmap.len()));
        }

        line_offsets.shrink_to_fit();
        log!(
            "Indexed {} lines in japanese dictionary",
            line_offsets.len()
        );

        Ok(JapaneseDictionary { mmap, line_offsets })
    }

    /// Get all entries of the dictionary
    pub fn entries(&self) -> impl Iterator<Item = DictionaryEntry> + '_ {
        self.line_offsets
            .iter()
            .enumerate()
            .filter_map(|(id, &(start, end))| {
                let line_bytes = &self.mmap[start..end];
                if let Ok(line) = std::str::from_utf8(line_bytes) {
                    self.parse_line(line, id)
                } else {
                    None
                }
            })
    }

    /// Method to search without loading all entries
    pub fn search(&self, query: &str) -> Vec<DictionaryEntry> {
        self.entries()
            .filter(|entry| entry.hanzi.contains(query) || entry.definition.contains(query))
            .collect()
    }

    /// Find entries for specific characters
    pub fn find_entries_for_chars(
        &self,
        search_chars: &std::collections::HashSet<&str>,
    ) -> HashMap<String, Vec<DictionaryEntry>> {
        let mut char_to_entries: HashMap<String, Vec<DictionaryEntry>> =
            HashMap::with_capacity(search_chars.len());

        for (id, &(start, end)) in self.line_offsets.iter().enumerate() {
            let line_bytes = &self.mmap[start..end];
            if let Ok(line) = std::str::from_utf8(line_bytes) {
                let contains_search_char = search_chars.iter().any(|&ch| line.contains(ch));
                if !contains_search_char {
                    continue;
                }

                if let Some(entry) = self.parse_line(line, id) {
                    let hanzi = entry.hanzi.trim();
                    if search_chars.contains(hanzi) {
                        char_to_entries
                            .entry(hanzi.to_string())
                            .or_default()
                            .push(entry);
                    }
                }
            }
        }

        char_to_entries
    }

    fn parse_line(&self, line: &str, id: usize) -> Option<DictionaryEntry> {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            return None;
        }

        if let Some(start_bracket) = trimmed_line.find('[') {
            if let Some(end_bracket) = trimmed_line.find(']') {
                let hanzi_part = &trimmed_line[..start_bracket].trim();
                let lecture = &trimmed_line[start_bracket + 1..end_bracket];
                let definitions = &trimmed_line[end_bracket + 1..];
                let hanzi = hanzi_part.split_whitespace().last().unwrap_or("").trim();

                if !hanzi.is_empty() {
                    return Some(DictionaryEntry {
                        id: id + 1,
                        hanzi: hanzi.to_string(),
                        lecture: lecture.to_string(),
                        definition: definitions.trim().to_string(),
                    });
                }
            }
        }
        None
    }
}

#[server(SearchDictionary, "/searchdictionary")]
pub async fn search_dictionary(
    chars_string: String,
    is_ch: bool,
) -> Result<Vec<Flashcard>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use std::collections::HashSet;

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
        let char_to_entries = ch_dictionary.find_entries_for_chars(&search_chars);

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
        let char_to_entries = jap_dictionary.find_entries_for_chars(&search_chars);

        for &jp in &chars_array {
            let trimmed_jp = jp.trim();
            if seen_chars.insert(trimmed_jp) {
                if let Some(entries) = char_to_entries.get(trimmed_jp) {
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
                        front: RwSignal::new(trimmed_jp.to_string()),
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

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn test_parse_ch_input() {
        assert_eq!(parse_ch_input(""), Vec::<&str>::new());
        assert_eq!(parse_ch_input("你,好"), vec!["你", "好"]);
        assert_eq!(parse_ch_input("你，好"), vec!["你", "好"]);
        assert_eq!(parse_ch_input("你好"), vec!["你好"]);
    }

    #[test]
    fn test_parse_jap_input() {
        assert_eq!(parse_jap_input(""), Vec::<&str>::new());
        assert_eq!(parse_jap_input("学,校"), vec!["学", "校"]);
        assert_eq!(parse_jap_input("学、校"), vec!["学", "校"]);
        assert_eq!(parse_jap_input("学校"), vec!["学校"]);
    }

    #[test]
    fn test_remove_whitespace() {
        assert_eq!(remove_whitespace(" a b c "), "abc");
        assert_eq!(remove_whitespace("漢 字"), "漢字");
        assert_eq!(remove_whitespace(""), "");
    }
}
