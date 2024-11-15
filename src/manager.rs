use crate::cheatsheet::CheatSheet;
use crate::highlighting::RustHighlighter;
use std::error::Error;

const BASICS: &str = include_str!("sheets/basics.rs");
const INTERMEDIATE: &str = include_str!("sheets/intermediate.rs");
const ADVANCED: &str = include_str!("sheets/advanced.rs");
const NALGEBRA: &str = include_str!("sheets/nalgebra.rs");

pub struct CheatSheetManager {
    sheets: Vec<(&'static str, &'static str)>,
    highlighter: RustHighlighter,
}

impl CheatSheetManager {
    pub fn new() -> Self {
        // Order here determines display order
        let sheets = vec![
            ("basics", BASICS),
            ("intermediate", INTERMEDIATE),
            ("advanced", ADVANCED),
            ("nalgebra", NALGEBRA),
        ];

        Self {
            sheets,
            highlighter: RustHighlighter::new(),
        }
    }

    fn format_name(name: &str) -> String {
        name.split(['_', '-'])
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        let mut word = first.to_uppercase().collect::<String>();
                        word.extend(chars.map(|c| c.to_lowercase().next().unwrap_or(c)));
                        word
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn get_sheet_by_index(&self, index: usize) -> Option<(&str, &str)> {
        self.sheets.get(index).copied()
    }

    pub fn show_available_sheets(&self) {
        for (idx, (sheet_name, content)) in self.sheets.iter().enumerate() {
            if let Ok(cheat_sheet) = CheatSheet::parse(content) {
                println!(
                    "\n{}",
                    self.highlighter.format_header(
                        &format!("{} - {}", idx, Self::format_name(sheet_name)),
                        true
                    )
                );
                self.print_sections(&cheat_sheet.sections);
            }
        }
    }

    pub fn show_sheet_outline(&self, index: usize) -> Result<(), Box<dyn Error>> {
        let (sheet_name, content) = self
            .get_sheet_by_index(index)
            .ok_or_else(|| format!("Could not find sheet at index {}", index))?;

        let cheat_sheet = CheatSheet::parse(content)?;
        println!(
            "{}",
            self.highlighter.format_header(
                &format!("{} - {}", index, Self::format_name(sheet_name)),
                true
            )
        );
        self.print_sections(&cheat_sheet.sections);
        Ok(())
    }

    fn print_sections(&self, sections: &[crate::cheatsheet::Section]) {
        for (i, section) in sections.iter().enumerate() {
            let prefix = if i == sections.len() - 1 {
                "└──"
            } else {
                "├──"
            };
            let header = format!("{} {}. {}", prefix, i + 1, section.title);
            println!("{}", self.highlighter.format_header(&header, false));
        }
    }

    pub fn show_section(
        &self,
        sheet_index: usize,
        section_number: &str,
    ) -> Result<(), Box<dyn Error>> {
        let (_, content) = self
            .get_sheet_by_index(sheet_index)
            .ok_or_else(|| format!("Could not find sheet at index {}", sheet_index))?;

        let cheat_sheet = CheatSheet::parse(content)?;
        let section_idx = section_number
            .parse::<usize>()
            .map_err(|_| "Section number must be a positive integer")?;

        if section_idx == 0 || section_idx > cheat_sheet.sections.len() {
            return Err("Invalid section number".into());
        }

        print!(
            "{}",
            self.highlighter
                .highlight(&cheat_sheet.sections[section_idx - 1].content)
        );
        Ok(())
    }

    pub fn show_full_sheet(&self, sheet_index: usize) -> Result<(), Box<dyn Error>> {
        let (_, content) = self
            .get_sheet_by_index(sheet_index)
            .ok_or_else(|| format!("Could not find sheet at index {}", sheet_index))?;

        println!("{}", self.highlighter.highlight(content));
        Ok(())
    }

    pub fn format_error(&self, error: &str) -> String {
        self.highlighter.format_error(error)
    }
}
