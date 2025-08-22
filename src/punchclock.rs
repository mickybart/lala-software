mod myr;

use std::fmt::Display;

pub(crate) use myr::Myr;

pub(crate) trait Report {
    fn parser(&self) -> ParsedEntries;
}

pub(crate) struct ParsedEntry {
    pub(crate) day: String,
    pub(crate) group: String,
    pub(crate) employee: String,
    pub(crate) total: String,
}

pub(crate) struct ParsedEntries(Vec<ParsedEntry>);

impl ParsedEntries {
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

impl Display for ParsedEntries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.0.iter() {
            writeln!(
                f,
                "{};{};{};{}",
                entry.day, entry.group, entry.employee, entry.total
            )?;
        }

        Ok(())
    }
}
