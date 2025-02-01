use std::{
    fs::File,
    io::{BufWriter, Error},
    ops::Range,
};

use crate::{
    editor::Annotation,
    prelude::*,
};
use super::{AnnotatedString, Highlighter, FileInfo};

use ropey::Rope;

#[derive(Default)]
pub struct BufferRope {
    text: Rope,
    file_info: FileInfo,
    dirty: bool,
}

impl BufferRope {
    pub fn load(file_name: &str) -> Result<Self, Error> {
        // Load a text file.
        let text = ropey::Rope::from_reader(
            File::open(file_name)?
        )?;

        Ok(Self {
            text,
            file_info: FileInfo::from(file_name),
            dirty: false,
        })
    }

    pub const fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub const fn get_file_info(&self) -> &FileInfo {
        &self.file_info
    }

    pub fn height(&self) -> LineIdx {
        // why -1 ?
        self.text.len_lines() - 1
    }

    pub const fn is_file_loaded(&self) -> bool {
        self.file_info.has_path()
    }

    pub fn is_empty(&self) -> bool {
        self.text.len_bytes() == 0
    }

    pub fn save_as(&mut self, file_name: &str) -> Result<(), Error> {
        let file_info = FileInfo::from(file_name);
        self.save_to_file(&file_info)?;
        self.file_info = file_info;
        self.dirty = false;
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), Error> {
        self.save_to_file(&self.file_info)?;
        self.dirty = false;
        Ok(())
    }

    fn save_to_file(&self, file_info: &FileInfo) -> Result<(), Error> {
        if let Some(file_path) = &file_info.get_path() {
            let file = File::create(file_path)?;
            self.text.write_to(
                BufWriter::new(file)
            )?;
        } else {
            #[cfg(debug_assertions)]
            {
                panic!("Attempting to save with no file path present");
            }
        }
        Ok(())
    }

    pub fn insert_char(&mut self, character: char, at: Location) {
        debug_assert!(at.line_idx <= self.height());
        if at.line_idx == self.height() {
            let last_char = self.text.len_chars();
            self.text.insert_char(last_char, character);
            self.dirty = true;
        } else {
            let char_index = self.text.line_to_char(at.line_idx);
            let start_idx = char_index + at.grapheme_idx;
            self.text.insert_char(start_idx, character);
            self.dirty = true;
        }
    }

    pub fn insert_newline(&mut self, at: Location) {
        if at.line_idx == self.height() {
            // add new line after last line
            let last_char = self.text.len_chars();
            self.text.insert(last_char, "\n");
            self.dirty = true;
        } else {
            let char_index = self.text.line_to_char(at.line_idx + 1);
            self.text.insert(char_index, "\n");
            self.dirty = true;
        }
    }

    pub fn delete(&mut self, at: Location) {
        let char_index = self.text.line_to_char(at.line_idx);
        let start_idx = char_index + at.grapheme_idx;
        self.text.remove(start_idx..1);
    }

    pub fn search_forward(&self, query: &str, from: Location) -> Option<Location> {
        None
    }

    pub fn search_backward(&self, query: &str, from: Location) -> Option<Location> {
        None
    }

    pub fn highlight(&self, idx: LineIdx, highlighter: &mut Highlighter) {
        let start_idx = self.text.line_to_char(idx);
        let end_idx = if idx + 1 == self.height() {
            self.text.len_chars()
        } else {
            self.text.line_to_char(idx + 1)
        };

        highlighter.highlight_string(idx, &self.text.slice(start_idx..end_idx).to_string());
    }

    pub fn get_highlighted_substring(
        &self,
        line_idx: LineIdx,
        range: Range<GraphemeIdx>,
        highlighter: &Highlighter,
    ) -> Option<AnnotatedString> {
        if line_idx >= self.height() {
            return None
        }

        let start_idx = self.text.line_to_char(line_idx);
        let end_idx = if line_idx + 1 == self.height() {
            self.text.len_chars()
        } else {
            self.text.line_to_char(line_idx + 1)
        };
        let line_str = self.text.slice(start_idx..end_idx).to_string();
        self.get_annotated_visible_substr(&line_str, range, Some(&highlighter.get_annotations(line_idx)));

        Some(AnnotatedString::from(&line_str))
    }

    pub fn grapheme_count(&self, idx: LineIdx) -> GraphemeIdx {
        if idx >= self.height() {
            return 0
        }

        let start_idx = self.text.line_to_char(idx);
        let end_idx = if idx + 1 == self.height() {
            self.text.len_chars()
        } else {
            self.text.line_to_char(idx + 1)
        };
        end_idx - start_idx - 1
    }

    pub fn width_until(&self, idx: LineIdx, until: GraphemeIdx) -> GraphemeIdx {
        until
    }

    fn get_annotated_visible_substr(
        &self,
        line_str: &str,
        range: Range<ColIdx>,
        annotations: Option<&Vec<Annotation>>,
    ) -> AnnotatedString {
        if range.start >= range.end {
            return AnnotatedString::default();
        }
        // Create a new annotated string
        let mut result = AnnotatedString::from(line_str);

        // Apply annotations for this string
        if let Some(annotations) = annotations {
            for annotation in annotations {
                result.add_annotation(annotation.annotation_type, annotation.start, annotation.end);
            }
        }

        // Insert replacement characters, and truncate if needed.
        // We do this backwards, otherwise the byte indices would be off in case a replacement character has a different width than the original character.

        // let mut fragment_start = self.width();
        // for fragment in self.fragments.iter().rev() {
        //     let fragment_end = fragment_start;
        //     fragment_start = fragment_start.saturating_sub(fragment.rendered_width.into());

        //     if fragment_start > range.end {
        //         continue; // No  processing needed if we haven't reached the visible range yet.
        //     }

        //     // clip right if the fragment is partially visible
        //     if fragment_start < range.end && fragment_end > range.end {
        //         result.replace(fragment.start, line_str.len(), "⋯");
        //         continue;
        //     } else if fragment_start == range.end {
        //         // Truncate right if we've reached the end of the visible range
        //         result.truncate_right_from(fragment.start);
        //         continue;
        //     }

        //     // Fragment ends at the start of the range: Remove the entire left side of the string (if not already at start of string)
        //     if fragment_end <= range.start {
        //         result.truncate_left_until(fragment.start.saturating_add(fragment.grapheme.len()));
        //         break; //End processing since all remaining fragments will be invisible.
        //     } else if fragment_start < range.start && fragment_end > range.start {
        //         // Fragment overlaps with the start of range: Remove the left side of the string and add an ellipsis
        //         result.replace(
        //             0,
        //             fragment.start.saturating_add(fragment.grapheme.len()),
        //             "⋯",
        //         );
        //         break; //End processing since all remaining fragments will be invisible.
        //     }

        //     // Fragment is fully within range: Apply replacement characters if appropriate
        //     if fragment_start >= range.start && fragment_end <= range.end {
        //         if let Some(replacement) = fragment.replacement {
        //             let start = fragment.start;
        //             let end = start.saturating_add(fragment.grapheme.len());
        //             result.replace(start, end, &replacement.to_string());
        //         }
        //     }
        // }

        result
    }
}
