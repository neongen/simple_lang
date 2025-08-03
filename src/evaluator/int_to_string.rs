/// Converts a 32-bit signed integer to its string representation.
pub fn int_to_string(value: i32) -> String {
    std::string::ToString::to_string(&value)
}
