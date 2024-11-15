use super::section::Section;
use std::error::Error;

pub struct CheatSheet {
    pub sections: Vec<Section>,
}

impl CheatSheet {
    pub fn parse(content: &str) -> Result<Self, Box<dyn Error>> {
        let lines: Vec<&str> = content.lines().collect();
        let section_starts = Self::find_section_starts(&lines);
        let sections = Self::build_sections(&lines, &section_starts)?;

        Ok(CheatSheet { sections })
    }

    fn find_section_starts(lines: &[&str]) -> Vec<usize> {
        lines
            .windows(2)
            .enumerate()
            .filter_map(|(i, window)| {
                let is_divider =
                    window[0].starts_with("# ----") || window[0].starts_with("// ----");
                let is_section = (window[1].starts_with("# ") || window[1].starts_with("// "))
                    && window[1]
                        .trim_start_matches(['#', '/', ' '])
                        .split_once(". ")
                        .is_some();

                if is_divider && is_section {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }

    fn build_sections(
        lines: &[&str],
        section_starts: &[usize],
    ) -> Result<Vec<Section>, Box<dyn Error>> {
        section_starts
            .iter()
            .enumerate()
            .map(|(i, &start_idx)| {
                let end_idx = section_starts.get(i + 1).copied().unwrap_or(lines.len());
                Self::create_section(lines, start_idx, end_idx)
            })
            .collect()
    }

    fn create_section(
        lines: &[&str],
        start_idx: usize,
        end_idx: usize,
    ) -> Result<Section, Box<dyn Error>> {
        let title_line = lines.get(start_idx + 1).ok_or("Missing section title")?;

        let section_title = title_line
            .trim_start_matches(['#', '/', ' '])
            .split_once(". ")
            .ok_or("Invalid section title format")?
            .1;

        let section_content = format!(
            "{}\n{}\n{}\n{}",
            lines[start_idx],
            title_line,
            lines[start_idx + 2],
            lines[start_idx + 3..end_idx].join("\n")
        );

        Ok(Section::new(section_title.to_string(), section_content))
    }
}
