///! Reads a source file from the filesystem and creates a SourceFile structure.
///!
///! This function validates the file path, reads the content, determines encoding,
///! and calculates the file size. It handles common file I/O errors and ensures
///! the file is readable as UTF-8 text.

/// Reads a source file from the given file path and returns a SourceFile structure.
///
/// Validates the file exists, reads its content as UTF-8, determines encoding,
/// and calculates size in bytes. Returns an error if the file cannot be read
/// or contains invalid UTF-8.
pub fn read_source_file(file_path: &str) -> std::result::Result<SourceFile, std::string::String> {
    if file_path.is_empty() {
        return std::result::Result::Err(std::string::String::from("File path cannot be empty"));
    }

    let content = match std::fs::read_to_string(file_path) {
        std::result::Result::Ok(content) => content,
        std::result::Result::Err(error) => {
            return std::result::Result::Err(std::format!("Failed to read file '{}': {}", file_path, error));
        }
    };

    let encoding = detect_encoding(&content);
    let size_bytes = calculate_content_size(&content);

    let source_file = SourceFile {
        path: std::string::String::from(file_path),
        content,
        encoding,
        size_bytes,
    };

    std::result::Result::Ok(source_file)
}

/// Detects the encoding of the given content string.
/// Assumes UTF-8 if successfully read; detects BOM if present.
fn detect_encoding(content: &str) -> std::string::String {
    if content.starts_with('\u{FEFF}') {
        std::string::String::from("UTF-8-BOM")
    } else {
        std::string::String::from("UTF-8")
    }
}

/// Calculates the byte size of the content string.
fn calculate_content_size(content: &str) -> u32 {
    content.as_bytes().len() as u32
}

/// Structure representing a loaded source file.
#[derive(Debug, PartialEq)]
pub struct SourceFile {
    pub path: std::string::String,
    pub content: std::string::String,
    pub encoding: std::string::String,
    pub size_bytes: u32,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_file_path() {
        let result = read_source_file("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "File path cannot be empty");
    }

    #[test]
    fn test_detect_encoding_regular_utf8() {
        let content = "Hello, world!";
        let encoding = detect_encoding(content);
        assert_eq!(encoding, "UTF-8");
    }

    #[test]
    fn test_detect_encoding_with_bom() {
        let content = "\u{FEFF}Hello, world!";
        let encoding = detect_encoding(content);
        assert_eq!(encoding, "UTF-8-BOM");
    }

    #[test]
    fn test_calculate_content_size() {
        let content = "Hello";
        let size = calculate_content_size(content);
        assert_eq!(size, 5);
    }

    #[test]
    fn test_calculate_content_size_unicode() {
        let content = "Hello 世界"; // Contains multi-byte UTF-8 characters
        let size = calculate_content_size(content);
        assert_eq!(size, 12); // "Hello " (6) + "世" (3) + "界" (3) = 12 bytes
    }
}