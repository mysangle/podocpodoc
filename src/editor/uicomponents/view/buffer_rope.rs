use std::{
    fs::File,
    io::{BufWriter, Error},
    ops::Range,
};

use crate::prelude::*;
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
        Some(AnnotatedString::from(&self.text.slice(start_idx..end_idx).to_string()))
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
}
