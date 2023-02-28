use std::fmt::Write;
use std::collections::VecDeque;
use anyhow::{self, Result};
use thiserror::Error;

use crate::field::Field;

pub const DELIMITER: &str = "|";

#[derive(Error, Debug)]
enum DrawError {
    #[error("field name is too long")]
    FieldNameTooLong,
}

pub struct SlicedCanvas {
    slice: VecDeque<String>,
    cursor: usize,
}

impl From<FieldDrawer> for SlicedCanvas {
    fn from(item: FieldDrawer) -> Self {
        // dedup delimiters
        let canvas = item.canvas.replace(
            format!("{DELIMITER}{DELIMITER}").as_str(),
            DELIMITER,
        );
        // 
        Self{
            slice: canvas
                    .as_bytes()
                    .chunks(64)
                    .map(|chunk| {
                        chunk
                            .iter()
                            .map(|&c| c as char)
                            .collect::<String>()
                    })
                    .collect::<VecDeque<String>>(),
            cursor: 0,
        }
    }
}

impl Iterator for SlicedCanvas {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.slice.pop_front();
        self.cursor += 1;
        item
    }
}

pub struct FieldDrawer {
    canvas: String,
    pub total_size: u64,
}

impl FieldDrawer {
    pub fn new() -> Self {
        Self {
            canvas: String::new(),
            total_size: 0,
        }
    }

    pub fn draw_field(&mut self, field: Field) -> Result<()> {
        let total_size: usize = (field.bitsize * 2) as usize;
        if field.name.len() + 2 > total_size {
            return Err(DrawError::FieldNameTooLong.into());
        }

        let padding_size = total_size - field.name.len();
        let leftpad_size: usize = (padding_size as f32 / 2_f32).floor() as usize;
        let rightpad_size: usize = ((padding_size as f32 / 2_f32).floor()) as usize + (padding_size % 2);

        write!(self.canvas, "{DELIMITER}")?;
        write!(self.canvas, "{}", " ".repeat(leftpad_size-1))?;
        write!(self.canvas, "{}", field.name)?;
        write!(self.canvas, "{}", " ".repeat(rightpad_size))?;
        write!(self.canvas, "{DELIMITER}")?;

        self.total_size += field.bitsize;

        Ok(())
    }
}
