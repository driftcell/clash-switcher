use unicode_width::UnicodeWidthStr;

/// Calculate display width and pad string to target width
pub fn pad_string(s: &str, width: usize) -> String {
    let display_width = s.width();
    if display_width >= width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - display_width))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_string_ascii() {
        assert_eq!(pad_string("hello", 10), "hello     ");
        assert_eq!(pad_string("test", 4), "test");
        assert_eq!(pad_string("foo", 3), "foo");
    }

    #[test]
    fn test_pad_string_exact_width() {
        assert_eq!(pad_string("exact", 5), "exact");
    }

    #[test]
    fn test_pad_string_overflow() {
        assert_eq!(pad_string("toolong", 5), "toolong");
    }

    #[test]
    fn test_pad_string_unicode() {
        // Chinese characters typically take 2 display widths
        assert_eq!(pad_string("中文", 6), "中文  ");
        // Mix of ASCII and Chinese
        assert_eq!(pad_string("a中", 5), "a中  ");
    }

    #[test]
    fn test_pad_string_emoji() {
        // Emojis typically take 2 display widths
        assert_eq!(pad_string("🚀", 4), "🚀  ");
        assert_eq!(pad_string("test🎉", 10), "test🎉    ");
    }

    #[test]
    fn test_pad_string_zero_width() {
        assert_eq!(pad_string("", 5), "     ");
    }

    #[test]
    fn test_pad_string_mixed_width() {
        // Japanese hiragana (2 width each)
        assert_eq!(pad_string("あいう", 8), "あいう  ");
        // Korean characters (2 width each)
        assert_eq!(pad_string("한글", 6), "한글  ");
    }
}
