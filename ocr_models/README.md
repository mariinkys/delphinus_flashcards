# OCR Models

OCR Models are provided by [oar-ocr](https://github.com/GreatV/oar-ocr) and are [Apache-2.0 licensed](https://github.com/GreatV/oar-ocr/blob/main/LICENSE).

Models used are:

### Text Detection Models
| Model Type     | Version  | Category | Model File                                                                                                      | Size    | Description                                    |
|----------------|----------|----------|-----------------------------------------------------------------------------------------------------------------|---------|------------------------------------------------|
| Text Detection | PP-OCRv5 | Mobile   | [`ppocrv5_mobile_det.onnx`](https://github.com/GreatV/oar-ocr/releases/download/v0.1.0/ppocrv5_mobile_det.onnx) | 4.8MB   | Mobile variant for real-time applications      |

### Text Recognition Models
| Model Type       | Version  | Language/Category | Model File                                                                                                              | Size   | Description                      |
|------------------|----------|-------------------|-------------------------------------------------------------------------------------------------------------------------|--------|----------------------------------|
| Text Recognition | PP-OCRv5 | Chinese/General   | [`ppocrv5_mobile_rec.onnx`](https://github.com/GreatV/oar-ocr/releases/download/v0.1.0/ppocrv5_mobile_rec.onnx)         | 16.5MB | Mobile variant                   |

### Character Dictionaries
| File Type            | Version        | Category | Model File                                                                                                | Size | Description                  |
|----------------------|----------------|----------|-----------------------------------------------------------------------------------------------------------|------|------------------------------|
| Character Dictionary | PP-OCRv5       | General  | [`ppocrv5_dict.txt`](https://github.com/GreatV/oar-ocr/releases/download/v0.1.0/ppocrv5_dict.txt)         | -    | For PP-OCRv5 models          |
