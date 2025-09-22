use super::Annotation;
use super::Line;
use crate::prelude::*;
pub trait SyntaxHighlighter {
    fn highlight(&mut self, idx: LineIdx, line: &Line);
    fn get_annotations(&self, idx: LineIdx) -> Option<&Vec<Annotation>>;
}
