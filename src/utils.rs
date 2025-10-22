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
