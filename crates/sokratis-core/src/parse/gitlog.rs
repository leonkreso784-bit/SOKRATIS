use crate::{Commit, ParseError};
pub struct Parsed {
    pub commits: Vec<Commit>,
    pub skipped_lines: usize,
}
pub fn parse_git_log(text: &str) -> Result<Parsed, ParseError> {
    let _ = text;
    todo!("cigla M1/3")
}
