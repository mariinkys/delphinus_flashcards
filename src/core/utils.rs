/// Checks if a file is a supported image format and returns its extension.
///
/// This function examines the MIME type of a web file and determines if it
/// represents a supported image format for processing. GIF and SVG formats
/// are explicitly excluded from support.
///
/// # Arguments
///
/// * `file` - A reference to a `web_sys::File` object to check
///
/// # Returns
///
/// * `Some(&'static str)` - The file extension as a string slice if the image
///   format is supported (png, jpeg, jpg, or webp)
/// * `None` - If the file is not a supported image format, or if it is a GIF
///   or SVG image
///
/// # Supported Formats
///
/// * PNG - returns "png"
/// * JPEG - returns "jpeg"
/// * JPG - returns "jpg"
/// * WEBP - returns "webp"
///
/// # Unsupported Formats
///
/// * GIF - returns `None`
/// * SVG - returns `None`
/// * All other types - returns `None`
pub fn is_extension_image(file: &web_sys::File) -> Option<&'static str> {
    match file.type_().as_str() {
        "image/png" => Some("png"),
        "image/jpeg" => Some("jpeg"),
        "image/jpg" => Some("jpg"),
        "image/webp" => Some("webp"),
        "image/gif" => None,
        "image/svg+xml" => None,
        _ => None,
    }
}
