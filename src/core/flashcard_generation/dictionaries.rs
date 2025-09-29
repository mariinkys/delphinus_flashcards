mod chinese_dict;
mod japanese_dict;

#[cfg(feature = "ssr")]
pub use chinese_dict::ChineseDictionary;
#[cfg(feature = "ssr")]
pub use japanese_dict::JapaneseDictionary;
