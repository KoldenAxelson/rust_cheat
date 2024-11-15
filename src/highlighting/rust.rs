use syntect::easy::HighlightLines;
use syntect::highlighting::{Color, Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

pub struct RustHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl RustHighlighter {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    // Creates a default style with white text on black background
    fn create_default_style() -> Style {
        Style {
            foreground: Color::WHITE,
            background: Color::BLACK,
            font_style: syntect::highlighting::FontStyle::empty(),
        }
    }

    // Checks if a text segment contains escape characters
    fn contains_escape_chars(text: &str) -> bool {
        text.contains('\\')
    }

    pub fn highlight(&self, code: &str) -> String {
        let syntax = self
            .syntax_set
            .find_syntax_by_extension("rs") // Changed from "py" to "rs"
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let theme = &self.theme_set.themes["Solarized (dark)"];
        let mut highlighter = HighlightLines::new(syntax, theme);
        let mut result = String::new();

        for line in LinesWithEndings::from(code) {
            // Special handling for comment headers
            if line.trim_start().starts_with("// ----")  // Changed from "# ----" to "// ----"
                || (line.trim_start().starts_with("// ") && line.contains(". "))
            // Changed from "# " to "// "
            {
                result.push_str(&self.format_header(line, false));
                continue;
            }

            match highlighter.highlight_line(line, &self.syntax_set) {
                Ok(ranges) => {
                    for (style, text) in ranges {
                        if Self::contains_escape_chars(text) {
                            // Use default style for escape characters
                            let default_style = Self::create_default_style();
                            result.push_str(&Self::style_to_ansi(default_style, text));
                        } else {
                            // Create a filtered style that only keeps the foreground color
                            let filtered_style = Style {
                                foreground: style.foreground,
                                background: Color::BLACK,
                                font_style: syntect::highlighting::FontStyle::empty(),
                            };
                            result.push_str(&Self::style_to_ansi(filtered_style, text));
                        }
                    }
                }
                Err(_) => {
                    result.push_str(line);
                }
            }
        }

        result
    }

    // Convert a Style and text to ANSI escaped string
    fn style_to_ansi(style: Style, text: &str) -> String {
        format!(
            "\x1b[38;5;{};{};{}m{}\x1b[0m",
            style.foreground.r, style.foreground.g, style.foreground.b, text
        )
    }

    pub fn format_header(&self, text: &str, is_title: bool) -> String {
        if is_title {
            format!("\x1b[36m{}\x1b[0m", text) // Cyan for titles
        } else {
            format!("\x1b[34m{}\x1b[0m", text) // Blue for other headers
        }
    }

    pub fn format_error(&self, error: &str) -> String {
        format!("\x1b[31mError: {}\x1b[0m", error) // Red for errors
    }
}

impl Default for RustHighlighter {
    fn default() -> Self {
        Self::new()
    }
}
