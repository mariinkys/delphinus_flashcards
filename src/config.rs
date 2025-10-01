use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelphinusConfig {
    pub disable_ocr: bool,
}

impl DelphinusConfig {
    pub fn get() -> Self {
        let disable_ocr = std::env::var("DISABLE_OCR")
            .map(|v| v.to_lowercase() == "true")
            .unwrap_or(false);

        Self { disable_ocr }
    }
}
