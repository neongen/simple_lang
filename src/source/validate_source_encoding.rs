//! Validates that source code content uses valid UTF-8 encoding.
//!
//! This function ensures that all characters in the source content are valid UTF-8
//! sequences and reports any encoding violations that would prevent compilation.
//! It checks for invalid byte sequences, overlong encodings, and surrogate pairs.
//! Essential for ensuring consistent text processing across the compilation pipeline.

/// Validates that the provided content string contains only valid UTF-8 encoded text.
/// 
/// This function performs comprehensive UTF-8 validation including checking for
/// invalid byte sequences, overlong encodings, and prohibited code points that
/// could cause issues during compilation or runtime execution.
pub fn validate_source_encoding(content: &str) -> std::result::Result<(), std::string::String> {
    // Check for null bytes which are not allowed in source code
    if has_null_bytes(content) {
        return std::result::Result::Err(std::string::String::from(
            "Source code contains null bytes which are not permitted"
        ));
    }

    // Check for invalid UTF-8 sequences by attempting to iterate over chars
    if let std::result::Result::Err(error_msg) = validate_utf8_sequences(content) {
        return std::result::Result::Err(error_msg);
    }

    // Check for problematic Unicode code points
    if let std::result::Result::Err(error_msg) = validate_unicode_code_points(content) {
        return std::result::Result::Err(error_msg);
    }

    // Check for byte order marks (BOM) which can cause parsing issues
    if has_byte_order_mark(content) {
        return std::result::Result::Err(std::string::String::from(
            "Source code contains Byte Order Mark (BOM) which is not supported"
        ));
    }

    std::result::Result::Ok(())
}

/// Checks if the content contains any null bytes.
fn has_null_bytes(content: &str) -> bool {
    content.as_bytes().contains(&0)
}

/// Validates UTF-8 byte sequences for correctness.
fn validate_utf8_sequences(content: &str) -> std::result::Result<(), std::string::String> {
    // Rust strings are guaranteed to be valid UTF-8, but we need to check
    // for any remaining edge cases by examining the raw bytes
    let bytes = content.as_bytes();
    
    match std::str::from_utf8(bytes) {
        std::result::Result::Ok(_) => std::result::Result::Ok(()),
        std::result::Result::Err(error) => {
            std::result::Result::Err(std::format!(
                "Invalid UTF-8 sequence at byte position {}: {}",
                error.valid_up_to(),
                error
            ))
        }
    }
}

/// Validates Unicode code points for source code compatibility.
fn validate_unicode_code_points(content: &str) -> std::result::Result<(), std::string::String> {
    for (index, ch) in content.char_indices() {
        // Check for surrogate code points (U+D800 to U+DFFF)
        if ch as u32 >= 0xD800 && ch as u32 <= 0xDFFF {
            return std::result::Result::Err(std::format!(
                "Invalid surrogate code point U+{:04X} at position {}",
                ch as u32,
                index
            ));
        }

        // Check for non-characters (U+FDD0 to U+FDEF and ending in FFFE or FFFF)
        let code_point = ch as u32;
        if (code_point >= 0xFDD0 && code_point <= 0xFDEF) ||
           (code_point & 0xFFFE) == 0xFFFE {
            return std::result::Result::Err(std::format!(
                "Non-character code point U+{:04X} at position {}",
                code_point,
                index
            ));
        }
    }

    std::result::Result::Ok(())
}

/// Checks if the content starts with a Byte Order Mark.
fn has_byte_order_mark(content: &str) -> bool {
    let bytes = content.as_bytes();
    
    // UTF-8 BOM: EF BB BF
    if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
        return true;
    }
    
    // UTF-16 BE BOM: FE FF
    if bytes.len() >= 2 && bytes[0] == 0xFE && bytes[1] == 0xFF {
        return true;
    }
    
    // UTF-16 LE BOM: FF FE
    if bytes.len() >= 2 && bytes[0] == 0xFF && bytes[1] == 0xFE {
        return true;
    }
    
    false
}