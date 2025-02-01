use super::Annotation;
use super::Line;
use crate::prelude::*;
pub trait SyntaxHighlighter {
    fn highlight(&mut self, idx: LineIdx, line: &Line);
    fn highlight_string(&mut self, idx: LineIdx, line_str: &str);
    fn get_annotations(&self, idx: LineIdx) -> Option<&Vec<Annotation>>;
}
